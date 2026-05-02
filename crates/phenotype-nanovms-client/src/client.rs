//! Main client implementation for nanovms.

use std::sync::Arc;
use std::time::Duration;

use tracing::{debug, info, instrument};

use crate::{
    transport::{CliTransport, MockTransport, Transport},
    NanovmsError, Output, Result, Sandbox, SandboxConfig, Snapshot, Tier,
};

/// Builder for creating a NanovmsClient.
pub struct ClientBuilder {
    transport: Option<Arc<dyn Transport>>,
    default_tier: Tier,
    default_timeout: Duration,
    ops_path: Option<String>,
    config_path: Option<std::path::PathBuf>,
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self {
            transport: None,
            default_tier: Tier::Wasm,
            default_timeout: Duration::from_secs(60),
            ops_path: None,
            config_path: None,
        }
    }

    /// Set a custom transport.
    pub fn transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }

    /// Use CLI transport with custom settings.
    pub fn cli_transport(mut self) -> Self {
        let transport = CliTransport::new()
            .with_timeout(self.default_timeout);
        
        let transport = if let Some(path) = &self.ops_path {
            transport.with_ops_path(path.clone())
        } else {
            transport
        };

        let transport = if let Some(config) = &self.config_path {
            transport.with_config(config.clone())
        } else {
            transport
        };

        self.transport = Some(Arc::new(transport));
        self
    }

    /// Use mock transport for testing.
    pub fn mock_transport(mut self) -> Self {
        self.transport = Some(Arc::new(MockTransport::new()));
        self
    }

    /// Set default tier.
    pub fn default_tier(mut self, tier: Tier) -> Self {
        self.default_tier = tier;
        self
    }

    /// Set default timeout.
    pub fn default_timeout(mut self, duration: Duration) -> Self {
        self.default_timeout = duration;
        self
    }

    /// Set OPS binary path.
    pub fn ops_path(mut self, path: &str) -> Self {
        self.ops_path = Some(path.into());
        self
    }

    /// Set config path.
    pub fn config_path(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.config_path = Some(path.into());
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<NanovmsClient> {
        let transport = self.transport.ok_or_else(|| {
            NanovmsError::InvalidConfig("Transport must be specified".to_string())
        })?;

        Ok(NanovmsClient {
            transport,
            default_tier: self.default_tier,
            default_timeout: self.default_timeout,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Main client for interacting with nanovms.
///
/// This client provides async methods for managing sandboxes,
/// executing commands, and managing the lifecycle of unikernel instances.
#[derive(Clone)]
pub struct NanovmsClient {
    transport: Arc<dyn Transport>,
    default_tier: Tier,
    default_timeout: Duration,
}

impl NanovmsClient {
    /// Create a new client with default CLI transport.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use phenotype_nanovms_client::NanovmsClient;
    ///
    /// let client = NanovmsClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            transport: Arc::new(CliTransport::new()),
            default_tier: Tier::Wasm,
            default_timeout: Duration::from_secs(60),
        }
    }

    /// Create a new client with mock transport for testing.
    pub fn new_mock() -> Self {
        Self {
            transport: Arc::new(MockTransport::new()),
            default_tier: Tier::Wasm,
            default_timeout: Duration::from_secs(60),
        }
    }

    /// Create a new client builder for custom configuration.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get the default tier.
    pub fn default_tier(&self) -> Tier {
        self.default_tier
    }

    /// Get the default timeout.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    /// Create a new sandbox with the given configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use phenotype_nanovms_client::{NanovmsClient, SandboxConfig, Tier};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = NanovmsClient::new();
    ///     let config = SandboxConfig::new("my-sandbox", Tier::Wasm);
    ///     let sandbox = client.create_sandbox(config).await?;
    ///     println!("Created sandbox: {}", sandbox.id);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, config))]
    pub async fn create_sandbox(&self, config: impl Into<SandboxConfig>) -> Result<Sandbox> {
        let config = config.into();
        info!("Creating sandbox '{}'", config.name);
        self.transport.create_sandbox(&config).await
    }

    /// Create a new sandbox with the given name and default tier.
    #[instrument(skip(self))]
    pub async fn create_sandbox_simple(&self, name: &str) -> Result<Sandbox> {
        let config = SandboxConfig::new(name, self.default_tier);
        self.create_sandbox(config).await
    }

    /// Create a sandbox with a specific tier.
    #[instrument(skip(self))]
    pub async fn create_sandbox_with_tier(
        &self,
        name: &str,
        tier: Tier,
    ) -> Result<Sandbox> {
        let config = SandboxConfig::new(name, tier);
        self.create_sandbox(config).await
    }

    /// Get a sandbox by ID.
    #[instrument(skip(self))]
    pub async fn get_sandbox(&self, id: &str) -> Result<Sandbox> {
        self.transport.get_sandbox(id).await
    }

    /// List all sandboxes.
    #[instrument(skip(self))]
    pub async fn list_sandboxes(&self) -> Result<Vec<Sandbox>> {
        self.transport.list_sandboxes().await
    }

    /// Start a sandbox.
    #[instrument(skip(self))]
    pub async fn start_sandbox(&self, id: &str) -> Result<Sandbox> {
        info!("Starting sandbox: {}", id);
        self.transport.start_sandbox(id).await
    }

    /// Stop a sandbox.
    #[instrument(skip(self))]
    pub async fn stop_sandbox(&self, id: &str) -> Result<Sandbox> {
        info!("Stopping sandbox: {}", id);
        self.transport.stop_sandbox(id).await
    }

    /// Delete a sandbox.
    #[instrument(skip(self))]
    pub async fn delete_sandbox(&self, id: &str) -> Result<()> {
        info!("Deleting sandbox: {}", id);
        self.transport.delete_sandbox(id).await
    }

    /// Execute a command in a sandbox.
    ///
    /// # Arguments
    ///
    /// * `sandbox` - The sandbox ID or reference
    /// * `command` - The command and arguments to execute
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use phenotype_nanovms_client::NanovmsClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = NanovmsClient::new();
    ///     let output = client.execute("sandbox-id", &["echo", "hello"]).await?;
    ///     println!("stdout: {}", output.stdout);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, command))]
    pub async fn execute(
        &self,
        sandbox: &str,
        command: &[&str],
    ) -> Result<Output> {
        let cmd: Vec<String> = command.iter().map(|s| s.to_string()).collect();
        
        debug!("Executing command in sandbox {}: {:?}", sandbox, cmd);
        self.transport.execute(sandbox, &cmd).await
    }

    /// Execute a shell command in a sandbox.
    #[instrument(skip(self))]
    pub async fn execute_shell(
        &self,
        sandbox: &str,
        command: &str,
    ) -> Result<Output> {
        let cmd = vec!["sh".to_string(), "-c".to_string(), command.to_string()];
        self.transport.execute(sandbox, &cmd).await
    }

    /// Create a snapshot of a sandbox.
    ///
    /// # Arguments
    ///
    /// * `name` - The sandbox ID or reference
    /// * `snapshot_name` - The name for the new snapshot
    #[instrument(skip(self))]
    pub async fn snapshot(
        &self,
        name: &str,
        snapshot_name: &str,
    ) -> Result<Snapshot> {
        info!(
            "Creating snapshot '{}' for sandbox: {}",
            snapshot_name, name
        );
        self.transport.create_snapshot(name, snapshot_name).await
    }

    /// List snapshots for a sandbox.
    #[instrument(skip(self))]
    pub async fn list_snapshots(&self, sandbox_id: &str) -> Result<Vec<Snapshot>> {
        self.transport.list_snapshots(sandbox_id).await
    }

    /// Restore a sandbox from a snapshot.
    #[instrument(skip(self))]
    pub async fn restore_snapshot(
        &self,
        sandbox_id: &str,
        snapshot_id: &str,
    ) -> Result<Sandbox> {
        self.transport
            .restore_snapshot(sandbox_id, snapshot_id)
            .await
    }

    /// Delete a snapshot.
    #[instrument(skip(self))]
    pub async fn delete_snapshot(
        &self,
        sandbox_id: &str,
        snapshot_id: &str,
    ) -> Result<()> {
        self.transport
            .delete_snapshot(sandbox_id, snapshot_id)
            .await
    }

    /// Wait for a sandbox to reach a specific state.
    #[instrument(skip(self))]
    pub async fn wait_for_state(
        &self,
        sandbox_id: &str,
        state: crate::SandboxState,
        timeout: Duration,
    ) -> Result<Sandbox> {
        let start = std::time::Instant::now();

        loop {
            let sandbox = self.get_sandbox(sandbox_id).await?;
            if sandbox.state == state {
                return Ok(sandbox);
            }

            if start.elapsed() > timeout {
                return Err(NanovmsError::Timeout(timeout));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

impl Default for NanovmsClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension methods for convenience.
#[async_trait::async_trait]
pub trait SandboxExt {
    /// Start the sandbox.
    async fn start(&self, client: &NanovmsClient) -> Result<Sandbox>;
    
    /// Stop the sandbox.
    async fn stop(&self, client: &NanovmsClient) -> Result<Sandbox>;
    
    /// Delete the sandbox.
    async fn delete(&self, client: &NanovmsClient) -> Result<()>;
    
    /// Execute a command in the sandbox.
    async fn execute(&self, client: &NanovmsClient, command: &[&str]) -> Result<Output>;
    
    /// Create a snapshot.
    async fn snapshot(&self, client: &NanovmsClient, name: &str) -> Result<Snapshot>;
}

#[async_trait::async_trait]
impl SandboxExt for Sandbox {
    async fn start(&self, client: &NanovmsClient) -> Result<Sandbox> {
        client.start_sandbox(&self.id).await
    }

    async fn stop(&self, client: &NanovmsClient) -> Result<Sandbox> {
        client.stop_sandbox(&self.id).await
    }

    async fn delete(&self, client: &NanovmsClient) -> Result<()> {
        client.delete_sandbox(&self.id).await
    }

    async fn execute(&self, client: &NanovmsClient, command: &[&str]) -> Result<Output> {
        client.execute(&self.id, command).await
    }

    async fn snapshot(&self, client: &NanovmsClient, name: &str) -> Result<Snapshot> {
        client.snapshot(&self.id, name).await
    }
}
