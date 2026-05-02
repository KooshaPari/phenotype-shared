//! Unit tests for nanovms-client

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::time::Duration;
    use std::str::FromStr;

    use crate::{
        Mount, NetworkConfig, SandboxConfig, SandboxState, Tier, Sandbox, SandboxExt,
        NanovmsClient, NanovmsError,
    };

    #[test]
    fn test_tier_default_values() {
        assert_eq!(Tier::Wasm.default_memory_mb(), 128);
        assert_eq!(Tier::Wasm.default_cpus(), 1);

        assert_eq!(Tier::Gvisor.default_memory_mb(), 256);
        assert_eq!(Tier::Gvisor.default_cpus(), 1);

        assert_eq!(Tier::Firecracker.default_memory_mb(), 512);
        assert_eq!(Tier::Firecracker.default_cpus(), 2);
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(format!("{}", Tier::Wasm), "wasm");
        assert_eq!(format!("{}", Tier::Gvisor), "gvisor");
        assert_eq!(format!("{}", Tier::Firecracker), "firecracker");
    }

    #[test]
    fn test_tier_from_str() {
        assert_eq!(Tier::from_str("wasm").unwrap(), Tier::Wasm);
        assert_eq!(Tier::from_str("WASM").unwrap(), Tier::Wasm);
        assert_eq!(Tier::from_str("tier1").unwrap(), Tier::Wasm);
        assert_eq!(Tier::from_str("tier_1").unwrap(), Tier::Wasm);

        assert_eq!(Tier::from_str("gvisor").unwrap(), Tier::Gvisor);
        assert_eq!(Tier::from_str("tier2").unwrap(), Tier::Gvisor);
        assert_eq!(Tier::from_str("tier_2").unwrap(), Tier::Gvisor);

        assert_eq!(Tier::from_str("firecracker").unwrap(), Tier::Firecracker);
        assert_eq!(Tier::from_str("tier3").unwrap(), Tier::Firecracker);
        assert_eq!(Tier::from_str("tier_3").unwrap(), Tier::Firecracker);

        assert!(matches!(
            Tier::from_str("invalid"),
            Err(NanovmsError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.name, "default");
        assert_eq!(config.tier, Tier::Wasm);
        assert_eq!(config.memory_mb, 128);
        assert_eq!(config.cpus, 1);
        assert!(config.env.is_empty());
        assert_eq!(config.workdir, PathBuf::from("/"));
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_sandbox_config_builder() {
        let config = SandboxConfig::new("my-app", Tier::Firecracker)
            .with_id("custom-123")
            .with_memory(1024)
            .with_cpus(4)
            .with_timeout(Duration::from_secs(120))
            .with_env("KEY1", "value1")
            .with_env("KEY2", "value2")
            .with_label("env", "production")
            .with_label("team", "backend");

        assert_eq!(config.id, Some("custom-123".to_string()));
        assert_eq!(config.name, "my-app");
        assert_eq!(config.tier, Tier::Firecracker);
        assert_eq!(config.memory_mb, 1024);
        assert_eq!(config.cpus, 4);
        assert_eq!(config.timeout, Duration::from_secs(120));
        assert_eq!(config.env.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(config.env.get("KEY2"), Some(&"value2".to_string()));
        assert_eq!(config.labels.get("env"), Some(&"production".to_string()));
        assert_eq!(config.labels.get("team"), Some(&"backend".to_string()));
    }

    #[test]
    fn test_sandbox_config_ensure_id() {
        let mut config = SandboxConfig::new("test", Tier::Wasm);
        assert!(config.id.is_none());

        let id = config.ensure_id().to_string();
        assert!(!id.is_empty());
        assert_eq!(config.id, Some(id.clone()));

        // Second call should return same ID
        let id2 = config.ensure_id().to_string();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_mount() {
        let mount = Mount::new("/host", "/guest");
        assert_eq!(mount.source, PathBuf::from("/host"));
        assert_eq!(mount.destination, PathBuf::from("/guest"));
        assert!(!mount.read_only);

        let ro_mount = Mount::new("/host", "/guest").read_only();
        assert!(ro_mount.read_only);
    }

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert!(config.enabled);
        assert!(config.ports.is_empty());
        assert_eq!(config.dns, vec!["8.8.8.8".to_string()]);
    }

    #[test]
    fn test_sandbox_creation() {
        let config = SandboxConfig::new("test-sandbox", Tier::Gvisor)
            .with_memory(512)
            .with_cpus(2);

        let sandbox = Sandbox::from_config(config.clone());

        assert!(!sandbox.id.is_empty());
        assert_eq!(sandbox.name, "test-sandbox");
        assert_eq!(sandbox.tier, Tier::Gvisor);
        assert_eq!(sandbox.state, SandboxState::Pending);
        assert!(sandbox.ip_address.is_none());
    }

    #[test]
    fn test_sandbox_state_serialization() {
        use serde_json;

        let states = vec![
            (SandboxState::Pending, "\"pending\""),
            (SandboxState::Running, "\"running\""),
            (SandboxState::Stopped, "\"stopped\""),
            (SandboxState::Deleted, "\"deleted\""),
        ];

        for (state, expected) in states {
            let json = serde_json::to_string(&state).unwrap();
            assert_eq!(json, expected);

            let parsed: SandboxState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    #[test]
    fn test_tier_serialization() {
        use serde_json;

        let tiers = vec![
            (Tier::Wasm, "\"w_a_s_m\""),
            (Tier::Gvisor, "\"gvisor\""),
            (Tier::Firecracker, "\"firecracker\""),
        ];

        for (tier, expected) in tiers {
            let json = serde_json::to_string(&tier).unwrap();
            assert_eq!(json, expected);

            let parsed: Tier = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, tier);
        }
    }

    #[test]
    fn test_error_display() {
        let err = NanovmsError::SandboxNotFound("test-id".to_string());
        assert_eq!(err.to_string(), "sandbox not found: test-id");

        let err = NanovmsError::SandboxAlreadyExists("test-id".to_string());
        assert_eq!(err.to_string(), "sandbox already exists: test-id");

        let err = NanovmsError::ExecutionFailed("command failed".to_string());
        assert_eq!(err.to_string(), "execution failed: command failed");

        let err = NanovmsError::InvalidConfig("bad config".to_string());
        assert_eq!(err.to_string(), "invalid configuration: bad config");

        let err = NanovmsError::Timeout(Duration::from_secs(30));
        assert_eq!(err.to_string(), "timeout after 30s");
    }

    #[tokio::test]
    async fn test_mock_client() {
        let client = NanovmsClient::new_mock();
        
        // Test creating sandbox
        let sandbox = client.create_sandbox_simple("test").await.unwrap();
        assert_eq!(sandbox.name, "test");
        assert_eq!(sandbox.tier, Tier::Wasm);

        // Test getting sandbox
        let fetched = client.get_sandbox(&sandbox.id).await.unwrap();
        assert_eq!(fetched.id, sandbox.id);

        // Test listing sandboxes
        let list = client.list_sandboxes().await.unwrap();
        assert_eq!(list.len(), 1);

        // Test stopping
        let stopped = client.stop_sandbox(&sandbox.id).await.unwrap();
        assert_eq!(stopped.state, SandboxState::Stopped);

        // Test starting
        let started = client.start_sandbox(&sandbox.id).await.unwrap();
        assert_eq!(started.state, SandboxState::Running);

        // Test execution
        let output = client.execute(&sandbox.id, &["echo", "hello"]).await.unwrap();
        assert_eq!(output.exit_code, 0);

        // Test snapshot
        let snapshot = client.snapshot(&sandbox.id, "v1.0").await.unwrap();
        assert_eq!(snapshot.name, "v1.0");
        assert_eq!(snapshot.sandbox_id, sandbox.id);

        // Test listing snapshots
        let snapshots = client.list_snapshots(&sandbox.id).await.unwrap();
        assert_eq!(snapshots.len(), 1);

        // Test deleting sandbox
        client.delete_sandbox(&sandbox.id).await.unwrap();

        // Verify deletion
        let result = client.get_sandbox(&sandbox.id).await;
        assert!(matches!(result, Err(NanovmsError::SandboxNotFound(_))));
    }

    #[tokio::test]
    async fn test_builder() {
        let client = NanovmsClient::builder()
            .mock_transport()
            .default_tier(Tier::Gvisor)
            .default_timeout(Duration::from_secs(300))
            .build()
            .unwrap();

        assert_eq!(client.default_tier(), Tier::Gvisor);
        assert_eq!(client.default_timeout(), Duration::from_secs(300));

        // Create sandbox should use Gvisor tier
        let sandbox = client.create_sandbox_simple("tier-test").await.unwrap();
        assert_eq!(sandbox.tier, Tier::Gvisor);
    }

    #[tokio::test]
    async fn test_sandbox_ext() {
        let client = NanovmsClient::new_mock();
        let sandbox = client.create_sandbox_simple("ext-test").await.unwrap();

        // Test stop via extension
        let stopped = sandbox.stop(&client).await.unwrap();
        assert_eq!(stopped.state, SandboxState::Stopped);

        // Test start via extension
        let started = stopped.start(&client).await.unwrap();
        assert_eq!(started.state, SandboxState::Running);

        // Test execute via extension
        let output = started.execute(&client, &["ls", "-la"]).await.unwrap();
        assert_eq!(output.exit_code, 0);

        // Test snapshot via extension
        let snapshot = started.snapshot(&client, "snapshot-1").await.unwrap();
        assert_eq!(snapshot.name, "snapshot-1");

        // Test delete via extension
        started.delete(&client).await.unwrap();

        // Verify deletion
        assert!(client.get_sandbox(&started.id).await.is_err());
    }

    #[tokio::test]
    async fn test_duplicate_sandbox_creation() {
        let client = NanovmsClient::new_mock();

        let config = SandboxConfig::new("duplicate", Tier::Wasm)
            .with_id("same-id");

        // First creation should succeed
        let sandbox1 = client.create_sandbox(config.clone()).await.unwrap();
        assert_eq!(sandbox1.id, "same-id");

        // Second creation with same ID should fail
        let result = client.create_sandbox(config).await;
        assert!(matches!(result, Err(NanovmsError::SandboxAlreadyExists(_))));
    }

    #[tokio::test]
    async fn test_execute_shell() {
        let client = NanovmsClient::new_mock();
        let sandbox = client.create_sandbox_simple("shell-test").await.unwrap();

        let output = client.execute_shell(&sandbox.id, "echo 'hello world'").await.unwrap();
        assert_eq!(output.exit_code, 0);
    }

    #[tokio::test]
    async fn test_wait_for_state() {
        let client = NanovmsClient::new_mock();
        let sandbox = client.create_sandbox_simple("wait-test").await.unwrap();

        // Stop the sandbox first
        client.stop_sandbox(&sandbox.id).await.unwrap();

        // Wait for stopped state
        let result = client.wait_for_state(&sandbox.id, SandboxState::Stopped, Duration::from_secs(5)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().state, SandboxState::Stopped);

        // Start it again
        client.start_sandbox(&sandbox.id).await.unwrap();

        // Wait for running state
        let result = client.wait_for_state(&sandbox.id, SandboxState::Running, Duration::from_secs(5)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_restore_snapshot() {
        let client = NanovmsClient::new_mock();
        let sandbox = client.create_sandbox_simple("restore-test").await.unwrap();
        let snapshot = client.snapshot(&sandbox.id, "v1").await.unwrap();

        let restored = client.restore_snapshot(&sandbox.id, &snapshot.id).await.unwrap();
        assert_eq!(restored.id, sandbox.id);
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let client = NanovmsClient::new_mock();
        let sandbox = client.create_sandbox_simple("snapshot-delete-test").await.unwrap();
        let snapshot = client.snapshot(&sandbox.id, "to-delete").await.unwrap();

        // Delete the snapshot
        client.delete_snapshot(&sandbox.id, &snapshot.id).await.unwrap();

        // List should be empty
        let snapshots = client.list_snapshots(&sandbox.id).await.unwrap();
        assert!(snapshots.is_empty());
    }
}
