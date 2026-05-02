//! Tests for nanovms-client

use std::time::Duration;

use phenotype_nanovms_client::{
    NanovmsClient, SandboxConfig, SandboxExt, Tier,
};

#[tokio::test]
async fn test_mock_client_creation() {
    let client = NanovmsClient::new_mock();
    assert_eq!(client.default_tier(), Tier::Wasm);
}

#[tokio::test]
async fn test_create_sandbox() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test-sandbox").await.unwrap();
    
    assert_eq!(sandbox.name, "test-sandbox");
    assert_eq!(sandbox.tier, Tier::Wasm);
    assert!(!sandbox.id.is_empty());
}

#[tokio::test]
async fn test_create_sandbox_with_config() {
    let client = NanovmsClient::new_mock();
    let config = SandboxConfig::new("my-sandbox", Tier::Gvisor)
        .with_memory(512)
        .with_cpus(2)
        .with_env("FOO", "bar");
    
    let sandbox = client.create_sandbox(config).await.unwrap();
    assert_eq!(sandbox.name, "my-sandbox");
    assert_eq!(sandbox.tier, Tier::Gvisor);
    assert_eq!(sandbox.config.memory_mb, 512);
    assert_eq!(sandbox.config.cpus, 2);
    assert_eq!(sandbox.config.env.get("FOO"), Some(&"bar".to_string()));
}

#[tokio::test]
async fn test_all_tiers() {
    let client = NanovmsClient::new_mock();
    
    // Test Tier 1: WASM
    let wasm = client.create_sandbox_with_tier("wasm-box", Tier::Wasm).await.unwrap();
    assert_eq!(wasm.tier, Tier::Wasm);
    
    // Test Tier 2: gVisor
    let gvisor = client.create_sandbox_with_tier("gvisor-box", Tier::Gvisor).await.unwrap();
    assert_eq!(gvisor.tier, Tier::Gvisor);
    
    // Test Tier 3: Firecracker
    let fc = client.create_sandbox_with_tier("firecracker-box", Tier::Firecracker).await.unwrap();
    assert_eq!(fc.tier, Tier::Firecracker);
}

#[tokio::test]
async fn test_list_sandboxes() {
    let client = NanovmsClient::new_mock();
    
    // Initially empty
    let list = client.list_sandboxes().await.unwrap();
    assert!(list.is_empty());
    
    // Create some sandboxes
    let _s1 = client.create_sandbox_simple("sandbox-1").await.unwrap();
    let _s2 = client.create_sandbox_simple("sandbox-2").await.unwrap();
    
    // Check list
    let list = client.list_sandboxes().await.unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn test_get_sandbox() {
    let client = NanovmsClient::new_mock();
    let created = client.create_sandbox_simple("test").await.unwrap();
    
    let fetched = client.get_sandbox(&created.id).await.unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, created.name);
}

#[tokio::test]
async fn test_get_sandbox_not_found() {
    let client = NanovmsClient::new_mock();
    
    let result = client.get_sandbox("non-existent-id").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), phenotype_nanovms_client::NanovmsError::SandboxNotFound(_)));
}

#[tokio::test]
async fn test_start_stop_sandbox() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    // Stop
    let stopped = client.stop_sandbox(&sandbox.id).await.unwrap();
    assert_eq!(stopped.state, phenotype_nanovms_client::SandboxState::Stopped);
    
    // Start
    let started = client.start_sandbox(&sandbox.id).await.unwrap();
    assert_eq!(started.state, phenotype_nanovms_client::SandboxState::Running);
}

#[tokio::test]
async fn test_delete_sandbox() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    // Delete
    client.delete_sandbox(&sandbox.id).await.unwrap();
    
    // Verify deleted
    let result = client.get_sandbox(&sandbox.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_execute_command() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    let output = client.execute(&sandbox.id, &["echo", "hello"]).await.unwrap();
    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("echo hello"));
}

#[tokio::test]
async fn test_execute_shell() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    let output = client.execute_shell(&sandbox.id, "echo hello").await.unwrap();
    assert_eq!(output.exit_code, 0);
}

#[tokio::test]
async fn test_snapshot() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    // Create snapshot
    let snapshot = client.snapshot(&sandbox.id, "v1.0").await.unwrap();
    assert_eq!(snapshot.sandbox_id, sandbox.id);
    assert_eq!(snapshot.name, "v1.0");
    
    // List snapshots
    let snapshots = client.list_snapshots(&sandbox.id).await.unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].id, snapshot.id);
}

#[tokio::test]
async fn test_restore_snapshot() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    let snapshot = client.snapshot(&sandbox.id, "v1.0").await.unwrap();
    
    let restored = client.restore_snapshot(&sandbox.id, &snapshot.id).await.unwrap();
    assert_eq!(restored.id, sandbox.id);
}

#[tokio::test]
async fn test_delete_snapshot() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    let snapshot = client.snapshot(&sandbox.id, "v1.0").await.unwrap();
    
    client.delete_snapshot(&sandbox.id, &snapshot.id).await.unwrap();
    
    let snapshots = client.list_snapshots(&sandbox.id).await.unwrap();
    assert!(snapshots.is_empty());
}

#[tokio::test]
async fn test_sandbox_ext_methods() {
    let client = NanovmsClient::new_mock();
    let sandbox = client.create_sandbox_simple("test").await.unwrap();
    
    // Test extension methods
    let stopped = sandbox.stop(&client).await.unwrap();
    assert_eq!(stopped.state, phenotype_nanovms_client::SandboxState::Stopped);
    
    let started = stopped.start(&client).await.unwrap();
    assert_eq!(started.state, phenotype_nanovms_client::SandboxState::Running);
    
    let output = started.execute(&client, &["ls"]).await.unwrap();
    assert_eq!(output.exit_code, 0);
    
    let _snapshot = started.snapshot(&client, "ext-test").await.unwrap();
    
    started.delete(&client).await.unwrap();
}

#[tokio::test]
async fn test_tier_default_resources() {
    assert_eq!(Tier::Wasm.default_memory_mb(), 128);
    assert_eq!(Tier::Gvisor.default_memory_mb(), 256);
    assert_eq!(Tier::Firecracker.default_memory_mb(), 512);
    
    assert_eq!(Tier::Wasm.default_cpus(), 1);
    assert_eq!(Tier::Gvisor.default_cpus(), 1);
    assert_eq!(Tier::Firecracker.default_cpus(), 2);
}

#[tokio::test]
async fn test_tier_from_str() {
    use std::str::FromStr;
    
    assert_eq!(Tier::from_str("wasm").unwrap(), Tier::Wasm);
    assert_eq!(Tier::from_str("WASM").unwrap(), Tier::Wasm);
    assert_eq!(Tier::from_str("tier1").unwrap(), Tier::Wasm);
    
    assert_eq!(Tier::from_str("gvisor").unwrap(), Tier::Gvisor);
    assert_eq!(Tier::from_str("tier2").unwrap(), Tier::Gvisor);
    
    assert_eq!(Tier::from_str("firecracker").unwrap(), Tier::Firecracker);
    assert_eq!(Tier::from_str("tier3").unwrap(), Tier::Firecracker);
    
    assert!(Tier::from_str("invalid").is_err());
}

#[tokio::test]
async fn test_tier_display() {
    assert_eq!(Tier::Wasm.to_string(), "wasm");
    assert_eq!(Tier::Gvisor.to_string(), "gvisor");
    assert_eq!(Tier::Firecracker.to_string(), "firecracker");
}

#[tokio::test]
async fn test_sandbox_config_builder() {
    let config = SandboxConfig::new("test", Tier::Firecracker)
        .with_id("custom-id")
        .with_memory(1024)
        .with_cpus(4)
        .with_timeout(Duration::from_secs(120))
        .with_env("KEY", "value")
        .with_label("app", "myapp");
    
    assert_eq!(config.id, Some("custom-id".to_string()));
    assert_eq!(config.memory_mb, 1024);
    assert_eq!(config.cpus, 4);
    assert_eq!(config.timeout, Duration::from_secs(120));
    assert_eq!(config.env.get("KEY"), Some(&"value".to_string()));
    assert_eq!(config.labels.get("app"), Some(&"myapp".to_string()));
}

#[tokio::test]
async fn test_client_builder() {
    let client = phenotype_nanovms_client::NanovmsClient::builder()
        .mock_transport()
        .default_tier(Tier::Gvisor)
        .default_timeout(Duration::from_secs(300))
        .build()
        .unwrap();
    
    assert_eq!(client.default_tier(), Tier::Gvisor);
    assert_eq!(client.default_timeout(), Duration::from_secs(300));
}

#[tokio::test]
async fn test_mount_configuration() {
    use phenotype_nanovms_client::Mount;
    
    let mount = Mount::new("/host/path", "/guest/path");
    assert!(!mount.read_only);
    
    let ro_mount = Mount::new("/host/path", "/guest/path").read_only();
    assert!(ro_mount.read_only);
}
