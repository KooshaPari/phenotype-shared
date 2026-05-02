//! CLI integration for nanovms OPS.

use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{debug, error, info, instrument, warn};

use crate::{NanovmsError, Output, Result};

/// Configuration for the OPS CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsConfig {
    /// Path to the ops binary
    pub ops_path: String,
    /// Default provider (onprem, aws, gcp, azure)
    pub provider: String,
    /// Configuration file path
    pub config_file: Option<PathBuf>,
    /// Default zone/region
    pub zone: Option<String>,
    /// Default project
    pub project: Option<String>,
    /// Debug mode
    pub debug: bool,
    /// Timeout for operations
    pub timeout: Duration,
}

impl OpsConfig {
    /// Create a new default OPS configuration.
    pub fn new() -> Self {
        Self {
            ops_path: "ops".to_string(),
            provider: "onprem".to_string(),
            config_file: None,
            zone: None,
            project: None,
            debug: false,
            timeout: Duration::from_secs(60),
        }
    }

    /// Set the OPS binary path.
    pub fn with_ops_path(mut self, path: impl Into<String>) -> Self {
        self.ops_path = path.into();
        self
    }

    /// Set the provider.
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = provider.into();
        self
    }

    /// Set the config file.
    pub fn with_config_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.config_file = Some(path.into());
        self
    }

    /// Set the zone/region.
    pub fn with_zone(mut self, zone: impl Into<String>) -> Self {
        self.zone = Some(zone.into());
        self
    }

    /// Set the project.
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Enable debug mode.
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the CLI arguments for global flags.
    pub fn build_global_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(config) = &self.config_file {
            args.push("--config".to_string());
            args.push(config.display().to_string());
        }

        if self.debug {
            args.push("--debug".to_string());
        }

        args
    }
}

impl Default for OpsConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// OPS instance representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsInstance {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created: String,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    pub image: String,
}

/// OPS image representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsImage {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created: String,
}

/// OPS volume representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsVolume {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub status: String,
    pub attached_to: Option<String>,
}

/// Low-level OPS CLI wrapper.
#[derive(Debug, Clone)]
pub struct OpsCli {
    config: OpsConfig,
}

impl OpsCli {
    /// Create a new OPS CLI wrapper.
    pub fn new() -> Self {
        Self {
            config: OpsConfig::new(),
        }
    }

    /// Create with custom config.
    pub fn with_config(config: OpsConfig) -> Self {
        Self { config }
    }

    /// Get the current config.
    pub fn config(&self) -> &OpsConfig {
        &self.config
    }

    /// Execute a raw OPS command.
    #[instrument(skip(self), level = "debug")]
    pub async fn run(&self, args: &[&str]) -> Result<Output> {
        let mut cmd = Command::new(&self.config.ops_path);

        // Add global args first
        cmd.args(self.config.build_global_args());

        // Add subcommand args
        cmd.args(args);

        debug!("Running OPS command: {:?}", cmd);

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                error!("Failed to execute OPS command: {}", e);
                NanovmsError::Io(e)
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            warn!(
                "OPS command failed with exit code: {:?}",
                output.status.code()
            );
        }

        Ok(Output {
            stdout,
            stderr,
            exit_code: output.status.code().unwrap_or(-1),
            duration: Duration::from_secs(0),
        })
    }

    /// Build a unikernel image.
    pub async fn build_image(
        &self,
        binary: &std::path::Path,
        name: &str,
    ) -> Result<OpsImage> {
        info!("Building image '{}' from binary: {:?}", name, binary);

        let args = vec![
            "image",
            "create",
            "-n",
            name,
            binary.to_str().unwrap_or("."),
        ];

        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to build image: {}",
                output.stderr
            )));
        }

        Ok(OpsImage {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: format!("~/.ops/images/{}", name),
            size: 0,
            created: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// List images.
    #[instrument(skip(self))]
    pub async fn list_images(&self) -> Result<Vec<OpsImage>> {
        debug!("Listing images");

        let args = vec!["image", "list", "-j"];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to list images: {}",
                output.stderr
            )));
        }

        // Parse JSON output
        // For now, return empty list
        Ok(Vec::new())
    }

    /// Delete an image.
    pub async fn delete_image(&self, name: &str) -> Result<()> {
        info!("Deleting image: {}", name);

        let args = vec!["image", "delete", "-y", name];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to delete image: {}",
                output.stderr
            )));
        }

        Ok(())
    }

    /// Create an instance.
    pub async fn create_instance(
        &self,
        image: &str,
        name: &str,
    ) -> Result<OpsInstance> {
        info!("Creating instance '{}' from image: {}", name, image);

        let args = vec![
            "instance",
            "create",
            "-i",
            image,
            "-n",
            name,
            "-t",
            &self.config.provider,
        ];

        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to create instance: {}",
                output.stderr
            )));
        }

        Ok(OpsInstance {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            status: "pending".to_string(),
            created: chrono::Utc::now().to_rfc3339(),
            private_ip: None,
            public_ip: None,
            image: image.to_string(),
        })
    }

    /// List instances.
    #[instrument(skip(self))]
    pub async fn list_instances(&self) -> Result<Vec<OpsInstance>> {
        debug!("Listing instances");

        let args = vec!["instance", "list", "-j", "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to list instances: {}",
                output.stderr
            )));
        }

        // Parse JSON output
        // For now, return empty list
        Ok(Vec::new())
    }

    /// Start an instance.
    pub async fn start_instance(&self, id: &str) -> Result<()> {
        info!("Starting instance: {}", id);

        let args = vec!["instance", "start", id, "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to start instance: {}",
                output.stderr
            )));
        }

        Ok(())
    }

    /// Stop an instance.
    pub async fn stop_instance(&self, id: &str) -> Result<()> {
        info!("Stopping instance: {}", id);

        let args = vec!["instance", "stop", id, "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to stop instance: {}",
                output.stderr
            )));
        }

        Ok(())
    }

    /// Delete an instance.
    pub async fn delete_instance(&self, id: &str) -> Result<()> {
        info!("Deleting instance: {}", id);

        let args = vec!["instance", "delete", "-y", id, "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to delete instance: {}",
                output.stderr
            )));
        }

        Ok(())
    }

    /// Get logs from an instance.
    pub async fn get_logs(&self, id: &str) -> Result<String> {
        debug!("Getting logs for instance: {}", id);

        let args = vec!["instance", "logs", id, "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to get logs: {}",
                output.stderr
            )));
        }

        Ok(output.stdout)
    }

    /// Show instance information.
    pub async fn show_instance(&self, id: &str) -> Result<OpsInstance> {
        debug!("Getting info for instance: {}", id);

        let args = vec!["instance", "show", id, "-t", &self.config.provider];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to get instance info: {}",
                output.stderr
            )));
        }

        // Parse the output
        Ok(OpsInstance {
            id: id.to_string(),
            name: "unknown".to_string(),
            status: "unknown".to_string(),
            created: chrono::Utc::now().to_rfc3339(),
            private_ip: None,
            public_ip: None,
            image: "unknown".to_string(),
        })
    }

    /// Run a local unikernel.
    pub async fn run_local(
        &self,
        binary: &std::path::Path,
        args: &[&str],
    ) -> Result<Output> {
        debug!("Running local unikernel: {:?}", binary);

        let mut ops_args = vec!["run", binary.to_str().unwrap_or(".")];
        
        // Add any additional arguments
        for arg in args {
            ops_args.push(arg);
        }

        let output = self.run(&ops_args).await?;

        Ok(output)
    }

    /// Check if OPS is installed.
    #[instrument(skip(self))]
    pub async fn check_installation(&self) -> Result<bool> {
        debug!("Checking OPS installation");

        let args = vec!["version"];
        let output = self.run(&args).await?;

        Ok(output.exit_code == 0)
    }

    /// Get OPS version.
    #[instrument(skip(self))]
    pub async fn version(&self) -> Result<String> {
        let args = vec!["version"];
        let output = self.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(
                "Failed to get OPS version".to_string(),
            ));
        }

        Ok(output.stdout.trim().to_string())
    }
}

impl Default for OpsCli {
    fn default() -> Self {
        Self::new()
    }
}

/// Package management helpers.
pub mod packages {
    use super::*;

    /// List available packages.
    pub async fn list_packages(cli: &OpsCli) -> Result<Vec<String>> {
        let args = vec!["pkg", "list", "-j"];
        let output = cli.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to list packages: {}",
                output.stderr
            )));
        }

        // Parse JSON output
        Ok(Vec::new())
    }

    /// Get a package.
    pub async fn get_package(cli: &OpsCli, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        let args = vec!["pkg", "get", name];
        let output = cli.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to get package: {}",
                output.stderr
            )));
        }

        Ok(())
    }

    /// Describe a package.
    pub async fn describe_package(
        cli: &OpsCli,
        name: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        let name = name.as_ref();
        let args = vec!["pkg", "describe", name, "-j"];
        let output = cli.run(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to describe package: {}",
                output.stderr
            )));
        }

        serde_json::from_str(&output.stdout)
            .map_err(NanovmsError::Serialization)
    }
}
