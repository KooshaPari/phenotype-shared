use super::*;

#[tokio::test]
async fn test_create_agent_command() {
    let handler = AgentCommandHandler {};
    let cmd = CreateAgent {
        name: "Test Agent".to_string(),
        capabilities: vec!["coding".to_string()],
    };

    let result = handler.handle_create_agent(cmd).await.unwrap();
    assert!(result.success);
    let dto = result.data.unwrap();
    assert_eq!(dto.name, "Test Agent");
    assert_eq!(dto.status, "idle");
}

#[tokio::test]
async fn test_create_agent_empty_name() {
    let handler = AgentCommandHandler {};
    let cmd = CreateAgent {
        name: "".to_string(),
        capabilities: vec![],
    };

    let result = handler.handle_create_agent(cmd).await.unwrap();
    assert!(!result.success);
}

#[tokio::test]
async fn test_create_task_command() {
    let handler = TaskCommandHandler;
    let cmd = CreateTask {
        name: "Test Task".to_string(),
        description: Some("A test task".to_string()),
        priority: "high".to_string(),
        required_capabilities: vec!["coding".to_string()],
    };

    let result = handler.handle_create_task(cmd).await.unwrap();
    assert!(result.success);
    assert_eq!(result.data.unwrap().status, "pending");
}

#[tokio::test]
async fn test_get_agent_query() {
    let handler = AgentQueryHandler;
    let query = GetAgent {
        agent_id: "agent_123".to_string(),
    };

    let result = handler.handle_get_agent(query).await.unwrap();
    assert!(result.success);
}
