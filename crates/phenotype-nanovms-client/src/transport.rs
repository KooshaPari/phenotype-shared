//! Transport layer for communicating with nanovms.

use async_trait::async_trait;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};

use crate::{NanovmsError, Output, Result, Sandbox, SandboxConfig, Snapshot};

/// Abstract transport trait for nanovms communication.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Create a new sandbox.
    async fn create_sandbox(&self, config: &SandboxConfig) -> Result<Sandbox>;

    /// Get sandbox by ID.
    async fn get_sandbox(&self, id: &str) -> Result<Sandbox>;

    /// List all sandboxes.
    async fn list_sandboxes(&self) -> Result<Vec<Sandbox>>;

    /// Start a sandbox.
    async fn start_sandbox(&self, id: &str) -> Result<Sandbox>;

    /// Stop a sandbox.
    async fn stop_sandbox(&self, id: &str) -> Result<Sandbox>;

    /// Delete a sandbox.
    async fn delete_sandbox(&self, id: &str) -> Result<()>;

    /// Execute a command in a sandbox.
    async fn execute(&self, sandbox_id: &str, command: &[String]) -> Result<Output>;

    /// Create a snapshot of a sandbox.
    async fn create_snapshot(&self, sandbox_id: &str, name: &str) -> Result<Snapshot>;

    /// List snapshots for a sandbox.
    async fn list_snapshots(&self, sandbox_id: &str) -> Result<Vec<Snapshot>>;

    /// Restore a sandbox from a snapshot.
    async fn restore_snapshot(&self, sandbox_id: &str, snapshot_id: &str) -> Result<Sandbox>;

    /// Delete a snapshot.
    async fn delete_snapshot(&self, sandbox_id: &str, snapshot_id: &str) -> Result<()>;
}

/// CLI-based transport using OPS commands.
#[derive(Debug, Clone)]
pub struct CliTransport {
    ops_path: String,
    config_path: Option<std::path::PathBuf>,
    timeout: Duration,
}

impl CliTransport {
    /// Create a new CLI transport with default settings.
    pub fn new() -> Self {
        Self {
            ops_path: "ops".to_string(),
            config_path: None,
            timeout: Duration::from_secs(60),
        }
    }

    /// Set a custom OPS binary path.
    pub fn with_ops_path(mut self, path: impl Into<String>) -> Self {
        self.ops_path = path.into();
        self
    }

    /// Set a custom config path.
    pub fn with_config(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.config_path = Some(path.into());
        self
    }

    /// Set operation timeout.
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Execute an OPS command with the given arguments.
    #[instrument(skip(self), level = "debug")]
    async fn run_ops(&self, args: &[&str]) -> Result<Output> {
        let mut cmd = Command::new(&self.ops_path);
        cmd.args(args);

        if let Some(config) = &self.config_path {
            cmd.arg("--config").arg(config);
        }

        debug!("Running ops command: {:?}", cmd);

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                error!("Failed to execute ops command: {}", e);
                NanovmsError::Io(e)
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(Output {
            stdout,
            stderr,
            exit_code: output.status.code().unwrap_or(-1),
            duration: Duration::from_secs(0),
        })
    }

    /// Execute an OPS command with timeout.
    #[instrument(skip(self), level = "debug")]
    async fn run_ops_with_timeout(&self, args: &[&str]) -> Result<Output> {
        timeout(self.timeout, self.run_ops(args))
            .await
            .map_err(|_| NanovmsError::Timeout(self.timeout))?
    }
}

impl Default for CliTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for CliTransport {
    #[instrument(skip(self, config))]
    async fn create_sandbox(&self, config: &SandboxConfig) -> Result<Sandbox> {
        info!("Creating sandbox '{}' with tier {:?}", config.name, config.tier);

        let mut args = vec!["instance", "create", "-n", &config.name];

        // Add tier-specific configuration
        match config.tier {
            crate::Tier::Wasm => {
                args.push("-p");
                args.push("wasm");
            }
            crate::Tier::Gvisor => {
                args.push("-p");
                args.push("gvisor");
            }
            crate::Tier::Firecracker => {
                args.push("-p");
                args.push("firecracker");
            }
        }

        // Add memory
        args.push("-m");
        let mem_str = format!("{}", config.memory_mb);
        args.push(&mem_str);

        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to create sandbox: {}",
                output.stderr
            )));
        }

        let mut sandbox = Sandbox::from_config(config.clone());
        sandbox.id = config.id.clone().unwrap_or_else(|| {
            // Parse ID from output
            output
                .stdout
                .lines()
                .find(|l| l.contains("instance"))
                .and_then(|l| l.split_whitespace().last())
                .map(|s| s.to_string())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
        });

        info!("Created sandbox with ID: {}", sandbox.id);
        Ok(sandbox)
    }

    #[instrument(skip(self))]
    async fn get_sandbox(&self, id: &str) -> Result<Sandbox> {
        debug!("Getting sandbox: {}", id);

        let args = vec!["instance", "list", "-j"];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::Transport(format!(
                "Failed to list instances: {}",
                output.stderr
            )));
        }

        // Parse JSON output to find the sandbox
        // This is a simplified implementation
        Err(NanovmsError::SandboxNotFound(id.to_string()))
    }

    #[instrument(skip(self))]
    async fn list_sandboxes(&self) -> Result<Vec<Sandbox>> {
        debug!("Listing all sandboxes");

        let args = vec!["instance", "list", "-j"];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::Transport(format!(
                "Failed to list instances: {}",
                output.stderr
            )));
        }

        // Parse JSON output
        // Return empty list for now - actual implementation would parse ops output
        Ok(Vec::new())
    }

    #[instrument(skip(self))]
    async fn start_sandbox(&self, id: &str) -> Result<Sandbox> {
        info!("Starting sandbox: {}", id);

        let args = vec!["instance", "start", id];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to start sandbox: {}",
                output.stderr
            )));
        }

        self.get_sandbox(id).await
    }

    #[instrument(skip(self))]
    async fn stop_sandbox(&self, id: &str) -> Result<Sandbox> {
        info!("Stopping sandbox: {}", id);

        let args = vec!["instance", "stop", id];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to stop sandbox: {}",
                output.stderr
            )));
        }

        self.get_sandbox(id).await
    }

    #[instrument(skip(self))]
    async fn delete_sandbox(&self, id: &str) -> Result<()> {
        info!("Deleting sandbox: {}", id);

        let args = vec!["instance", "delete", "-y", id];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to delete sandbox: {}",
                output.stderr
            )));
        }

        info!("Deleted sandbox: {}", id);
        Ok(())
    }

    #[instrument(skip(self, command))]
    async fn execute(&self, sandbox_id: &str, command: &[String]) -> Result<Output> {
        debug!("Executing command in sandbox {}: {:?}", sandbox_id, command);

        // OPS doesn't directly support command execution in instances
        // This would require custom implementation or SSH into the instance
        warn!("Direct execution not supported via CLI transport - using stub");

        Ok(Output {
            stdout: String::new(),
            stderr: "Execution via CLI transport not implemented".to_string(),
            exit_code: -1,
            duration: Duration::from_secs(0),
        })
    }

    #[instrument(skip(self))]
    async fn create_snapshot(&self, sandbox_id: &str, name: &str) -> Result<Snapshot> {
        info!("Creating snapshot '{}' for sandbox: {}", name, sandbox_id);

        let args = vec!["image", "create", "--instance", sandbox_id, "-n", name];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to create snapshot: {}",
                output.stderr
            )));
        }

        Ok(Snapshot {
            id: uuid::Uuid::new_v4().to_string(),
            sandbox_id: sandbox_id.to_string(),
            name: name.to_string(),
            created_at: chrono::Utc::now(),
            size_bytes: 0,
            labels: std::collections::HashMap::new(),
        })
    }

    #[instrument(skip(self))]
    async fn list_snapshots(&self, sandbox_id: &str) -> Result<Vec<Snapshot>> {
        debug!("Listing snapshots for sandbox: {}", sandbox_id);

        let args = vec!["image", "list", "-j"];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::Transport(format!(
                "Failed to list snapshots: {}",
                output.stderr
            )));
        }

        // Parse JSON output
        Ok(Vec::new())
    }

    #[instrument(skip(self))]
    async fn restore_snapshot(&self, _sandbox_id: &str, _snapshot_id: &str) -> Result<Sandbox> {
        warn!("Snapshot restore not implemented via CLI transport");
        Err(NanovmsError::Unknown(
            "Snapshot restore not implemented".to_string(),
        ))
    }

    #[instrument(skip(self))]
    async fn delete_snapshot(&self, _sandbox_id: &str, snapshot_id: &str) -> Result<()> {
        info!("Deleting snapshot: {}", snapshot_id);

        let args = vec!["image", "delete", "-y", snapshot_id];
        let output = self.run_ops_with_timeout(&args).await?;

        if output.exit_code != 0 {
            return Err(NanovmsError::ExecutionFailed(format!(
                "Failed to delete snapshot: {}",
                output.stderr
            )));
        }

        Ok(())
    }
}

/// In-memory transport for testing.
#[derive(Debug, Default)]
pub struct MockTransport {
    sandboxes: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Sandbox>>>,
    snapshots: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Snapshot>>>,
}

impl MockTransport {
    /// Create a new mock transport.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn create_sandbox(&self, config: &SandboxConfig) -> Result<Sandbox> {
        let mut sandbox = Sandbox::from_config(config.clone());
        sandbox.state = crate::SandboxState::Running;

        let mut sandboxes = self.sandboxes.write().await;
        if sandboxes.contains_key(&sandbox.id) {
            return Err(NanovmsError::SandboxAlreadyExists(sandbox.id.clone()));
        }

        sandboxes.insert(sandbox.id.clone(), sandbox.clone());
        Ok(sandbox)
    }

    async fn get_sandbox(&self, id: &str) -> Result<Sandbox> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes
            .get(id)
            .cloned()
            .ok_or_else(|| NanovmsError::SandboxNotFound(id.to_string()))
    }

    async fn list_sandboxes(&self) -> Result<Vec<Sandbox>> {
        let sandboxes = self.sandboxes.read().await;
        Ok(sandboxes.values().cloned().collect())
    }

    async fn start_sandbox(&self, id: &str) -> Result<Sandbox> {
        let mut sandboxes = self.sandboxes.write().await;
        let sandbox = sandboxes
            .get_mut(id)
            .ok_or_else(|| NanovmsError::SandboxNotFound(id.to_string()))?;
        sandbox.state = crate::SandboxState::Running;
        Ok(sandbox.clone())
    }

    async fn stop_sandbox(&self, id: &str) -> Result<Sandbox> {
        let mut sandboxes = self.sandboxes.write().await;
        let sandbox = sandboxes
            .get_mut(id)
            .ok_or_else(|| NanovmsError::SandboxNotFound(id.to_string()))?;
        sandbox.state = crate::SandboxState::Stopped;
        Ok(sandbox.clone())
    }

    async fn delete_sandbox(&self, id: &str) -> Result<()> {
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes
            .remove(id)
            .ok_or_else(|| NanovmsError::SandboxNotFound(id.to_string()))?;
        Ok(())
    }

    async fn execute(&self, _sandbox_id: &str, command: &[String]) -> Result<Output> {
        let cmd_str = command.join(" ");
        Ok(Output {
            stdout: format!("Executed: {}", cmd_str),
            stderr: String::new(),
            exit_code: 0,
            duration: Duration::from_millis(100),
        })
    }

    async fn create_snapshot(&self, sandbox_id: &str, name: &str) -> Result<Snapshot> {
        let _sandbox = self.get_sandbox(sandbox_id).await?;
        let snapshot = Snapshot {
            id: uuid::Uuid::new_v4().to_string(),
            sandbox_id: sandbox_id.to_string(),
            name: name.to_string(),
            created_at: chrono::Utc::now(),
            size_bytes: 1024 * 1024 * 10, // 10 MB mock size
            labels: std::collections::HashMap::new(),
        };

        let mut snapshots = self.snapshots.write().await;
        snapshots.insert(snapshot.id.clone(), snapshot.clone());
        Ok(snapshot)
    }

    async fn list_snapshots(&self, sandbox_id: &str) -> Result<Vec<Snapshot>> {
        let snapshots = self.snapshots.read().await;
        Ok(snapshots
            .values()
            .filter(|s| s.sandbox_id == sandbox_id)
            .cloned()
            .collect())
    }

    async fn restore_snapshot(&self, sandbox_id: &str, snapshot_id: &str) -> Result<Sandbox> {
        let snapshots = self.snapshots.read().await;
        let _snapshot = snapshots
            .get(snapshot_id)
            .ok_or_else(|| NanovmsError::Unknown("Snapshot not found".to_string()))?;

        let mut sandboxes = self.sandboxes.write().await;
        let sandbox = sandboxes
            .get_mut(sandbox_id)
            .ok_or_else(|| NanovmsError::SandboxNotFound(sandbox_id.to_string()))?;

        sandbox.updated_at = chrono::Utc::now();
        Ok(sandbox.clone())
    }

    async fn delete_snapshot(&self, _sandbox_id: &str, snapshot_id: &str) -> Result<()> {
        let mut snapshots = self.snapshots.write().await;
        snapshots
            .remove(snapshot_id)
            .ok_or_else(|| NanovmsError::Unknown("Snapshot not found".to_string()))?;
        Ok(())
    }
}
