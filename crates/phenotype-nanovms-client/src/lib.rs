//! # nanovms-client
//!
//! A Rust client library for NanoVMs unikernel orchestration.
//!
//! This library provides async APIs for managing NanoVMs instances,
//! including sandbox creation, command execution, and lifecycle management.
//!
//! ## Features
//!
//! - **Tier 1**: WebAssembly (Wasm) sandboxes for lightweight isolation
//! - **Tier 2**: gVisor-based sandboxes for enhanced security
//! - **Tier 3**: Firecracker microVMs for full virtualization
//!
//! ## Example
//!
//! ```rust,no_run
//! use phenotype_nanovms_client::{NanovmsClient, Tier, SandboxConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = NanovmsClient::new();
//!     
//!     let config = SandboxConfig::new("my-sandbox", Tier::Wasm);
//!     let sandbox = client.create_sandbox(config).await?;
//!     
//!     let output = client.execute(&sandbox.id, &["echo", "Hello from nanovms!"]).await?;
//!     println!("Output: {}", output.stdout);
//!     
//!     client.delete_sandbox(&sandbox.id).await?;
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub mod cli;
pub mod client;
pub mod models;
pub mod transport;

#[cfg(test)]
mod lib_tests;

pub use client::{NanovmsClient, SandboxExt};
pub use models::*;

/// Errors that can occur when using the nanovms client.
#[derive(Error, Debug)]
pub enum NanovmsError {
    #[error("sandbox not found: {0}")]
    SandboxNotFound(String),
    
    #[error("sandbox already exists: {0}")]
    SandboxAlreadyExists(String),
    
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("transport error: {0}")]
    Transport(String),
    
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("timeout after {0:?}")]
    Timeout(Duration),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for nanovms operations.
pub type Result<T> = std::result::Result<T, NanovmsError>;

/// Represents the isolation tier for a sandbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    /// WebAssembly sandbox - lightweight, fast startup
    Wasm,
    /// gVisor sandbox - enhanced security with system call filtering
    Gvisor,
    /// Firecracker microVM - full virtualization
    Firecracker,
}

impl Tier {
    /// Returns the default memory allocation for this tier in MB.
    pub fn default_memory_mb(&self) -> u32 {
        match self {
            Tier::Wasm => 128,
            Tier::Gvisor => 256,
            Tier::Firecracker => 512,
        }
    }
    
    /// Returns the default CPU allocation for this tier.
    pub fn default_cpus(&self) -> u32 {
        match self {
            Tier::Wasm => 1,
            Tier::Gvisor => 1,
            Tier::Firecracker => 2,
        }
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Wasm => write!(f, "wasm"),
            Tier::Gvisor => write!(f, "gvisor"),
            Tier::Firecracker => write!(f, "firecracker"),
        }
    }
}

impl std::str::FromStr for Tier {
    type Err = NanovmsError;
    
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "wasm" | "tier1" | "tier_1" => Ok(Tier::Wasm),
            "gvisor" | "tier2" | "tier_2" => Ok(Tier::Gvisor),
            "firecracker" | "tier3" | "tier_3" => Ok(Tier::Firecracker),
            _ => Err(NanovmsError::InvalidConfig(format!("unknown tier: {}", s))),
        }
    }
}

/// Configuration for creating a new sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Unique identifier for the sandbox (auto-generated if not provided)
    pub id: Option<String>,
    /// Human-readable name
    pub name: String,
    /// Isolation tier
    pub tier: Tier,
    /// Memory limit in MB
    pub memory_mb: u32,
    /// Number of CPUs
    pub cpus: u32,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Working directory inside sandbox
    pub workdir: PathBuf,
    /// Timeout for operations
    pub timeout: Duration,
    /// Network configuration
    pub network: NetworkConfig,
    /// Volume mounts
    pub mounts: Vec<Mount>,
    /// Additional metadata
    pub labels: HashMap<String, String>,
}

impl SandboxConfig {
    /// Create a new sandbox configuration with defaults.
    pub fn new(name: impl Into<String>, tier: Tier) -> Self {
        Self {
            id: None,
            name: name.into(),
            tier,
            memory_mb: tier.default_memory_mb(),
            cpus: tier.default_cpus(),
            env: HashMap::new(),
            workdir: PathBuf::from("/"),
            timeout: Duration::from_secs(30),
            network: NetworkConfig::default(),
            mounts: Vec::new(),
            labels: HashMap::new(),
        }
    }
    
    /// Set a custom ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    
    /// Set memory limit in MB.
    pub fn with_memory(mut self, mb: u32) -> Self {
        self.memory_mb = mb;
        self
    }
    
    /// Set CPU count.
    pub fn with_cpus(mut self, count: u32) -> Self {
        self.cpus = count;
        self
    }
    
    /// Set timeout.
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }
    
    /// Add an environment variable.
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }
    
    /// Add a volume mount.
    pub fn with_mount(mut self, mount: Mount) -> Self {
        self.mounts.push(mount);
        self
    }
    
    /// Add a label.
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }
    
    /// Generate a unique ID if not set.
    pub fn ensure_id(&mut self) -> &str {
        if self.id.is_none() {
            self.id = Some(Uuid::new_v4().to_string());
        }
        self.id.as_ref().unwrap()
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self::new("default", Tier::Wasm)
    }
}

/// Network configuration for a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub ports: Vec<u16>,
    pub dns: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ports: Vec::new(),
            dns: vec!["8.8.8.8".to_string()],
        }
    }
}

/// Volume mount configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub read_only: bool,
}

impl Mount {
    pub fn new(source: impl Into<PathBuf>, destination: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
            read_only: false,
        }
    }
    
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }
}

/// Output from a command execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
}

/// Sandbox state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxState {
    Pending,
    Creating,
    Running,
    Paused,
    Stopped,
    Deleting,
    Deleted,
    Error,
}

/// Sandbox metadata and status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sandbox {
    pub id: String,
    pub name: String,
    pub tier: Tier,
    pub state: SandboxState,
    pub config: SandboxConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
}

impl Sandbox {
    /// Create a new sandbox from config.
    pub fn from_config(config: SandboxConfig) -> Self {
        let id = config.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
        let now = chrono::Utc::now();
        
        Self {
            id,
            name: config.name.clone(),
            tier: config.tier,
            state: SandboxState::Pending,
            config,
            created_at: now,
            updated_at: now,
            ip_address: None,
        }
    }
}

/// Session snapshot for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub sandbox_id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub labels: HashMap<String, String>,
}

/// Trait for nanovms transport implementations.
#[async_trait::async_trait]
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
