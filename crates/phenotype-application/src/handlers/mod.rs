//!
//! # Use Case Handlers
//!
//! Handlers implement the **application layer use cases**.
//! Each handler takes a command/query and coordinates domain + ports.
//!
//! ## Handler Responsibilities (Clean Architecture)
//!
//! 1. **Validation**: Validate input (command/query)
//! 2. **Authorization**: Check permissions (via inbound port)
//! 3. **Domain Logic**: Delegate to domain layer
//! 4. **Persistence**: Use outbound ports to save changes
//! 5. **Events**: Dispatch domain events to event bus
//! 6. **DTO**: Return appropriate response
//!
//! ## Transaction Boundaries
//!
//! Each handler runs in a single transaction. For multi-aggregate operations,
//! use the **Unit of Work** pattern via the repository port.

use crate::commands::*;
use crate::dto::*;
use crate::queries::*;
use crate::{ApplicationError, CommandResult, QueryResult};
use std::sync::Arc;

/// Agent command handler.
/// Coordinates agent-related commands with domain and ports.
pub struct AgentCommandHandler {
    // In the full implementation, this would hold references to ports
    // For now, we define the interface
}

impl AgentCommandHandler {
    /// Handles CreateAgent command.
    #[tracing::instrument(name = "handler.create_agent", skip(self))]
    pub async fn handle_create_agent(
        &self,
        cmd: CreateAgent,
    ) -> Result<CommandResult<AgentDto>, ApplicationError> {
        // 1. Validate command
        if cmd.name.trim().is_empty() {
            return Ok(CommandResult::err("Name cannot be empty".to_string()));
        }

        // 2. Build domain entity (via factory service in full impl)
        let dto = AgentDto {
            id: format!("agent_{}", uuid::Uuid::new_v4()),
            name: cmd.name,
            status: "idle".to_string(),
            capabilities: cmd.capabilities,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // 3. Return success with DTO
        Ok(CommandResult::ok(dto, None))
    }

    /// Handles UpdateAgentStatus command.
    #[tracing::instrument(name = "handler.update_agent_status", skip(self))]
    pub async fn handle_update_agent_status(
        &self,
        cmd: UpdateAgentStatus,
    ) -> Result<CommandResult<AgentDto>, ApplicationError> {
        // Validation
        if cmd.agent_id.trim().is_empty() {
            return Ok(CommandResult::err("Agent ID cannot be empty".to_string()));
        }

        let valid_statuses = ["idle", "active", "busy", "paused", "stopped", "error"];
        if !valid_statuses.contains(&cmd.status.as_str()) {
            return Ok(CommandResult::err(format!("Invalid status: {}", cmd.status)));
        }

        // Return mock response (full impl would fetch, update, save)
        Ok(CommandResult::ok(
            AgentDto {
                id: cmd.agent_id,
                name: "Agent".to_string(),
                status: cmd.status,
                capabilities: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            None,
        ))
    }
}

/// Task command handler.
pub struct TaskCommandHandler;

impl TaskCommandHandler {
    /// Handles CreateTask command.
    #[tracing::instrument(name = "handler.create_task", skip(self))]
    pub async fn handle_create_task(
        &self,
        cmd: CreateTask,
    ) -> Result<CommandResult<TaskDto>, ApplicationError> {
        if cmd.name.trim().is_empty() {
            return Ok(CommandResult::err("Task name cannot be empty".to_string()));
        }

        let dto = TaskDto {
            id: format!("task_{}", uuid::Uuid::new_v4()),
            name: cmd.name,
            description: cmd.description,
            status: "pending".to_string(),
            priority: cmd.priority,
            assigned_agent_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
        };

        Ok(CommandResult::ok(dto, None))
    }

    /// Handles AssignTask command.
    #[tracing::instrument(name = "handler.assign_task", skip(self))]
    pub async fn handle_assign_task(
        &self,
        cmd: AssignTask,
    ) -> Result<CommandResult<TaskDto>, ApplicationError> {
        if cmd.task_id.trim().is_empty() || cmd.agent_id.trim().is_empty() {
            return Ok(CommandResult::err("Task ID and Agent ID are required".to_string()));
        }

        Ok(CommandResult::ok(
            TaskDto {
                id: cmd.task_id,
                name: "Task".to_string(),
                description: None,
                status: "assigned".to_string(),
                priority: "medium".to_string(),
                assigned_agent_id: Some(cmd.agent_id),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                completed_at: None,
            },
            None,
        ))
    }

    /// Handles CompleteTask command.
    #[tracing::instrument(name = "handler.complete_task", skip(self))]
    pub async fn handle_complete_task(
        &self,
        cmd: CompleteTask,
    ) -> Result<CommandResult<TaskDto>, ApplicationError> {
        if cmd.task_id.trim().is_empty() {
            return Ok(CommandResult::err("Task ID cannot be empty".to_string()));
        }

        let now = chrono::Utc::now().to_rfc3339();
        Ok(CommandResult::ok(
            TaskDto {
                id: cmd.task_id,
                name: "Completed Task".to_string(),
                description: None,
                status: "completed".to_string(),
                priority: "medium".to_string(),
                assigned_agent_id: None,
                created_at: now.clone(),
                updated_at: now.clone(),
                completed_at: Some(now),
            },
            None,
        ))
    }
}

/// Agent query handler.
pub struct AgentQueryHandler;

impl AgentQueryHandler {
    /// Handles GetAgent query.
    #[tracing::instrument(name = "handler.get_agent", skip(self))]
    pub async fn handle_get_agent(
        &self,
        query: GetAgent,
    ) -> Result<QueryResult<AgentDto>, ApplicationError> {
        if query.agent_id.trim().is_empty() {
            return Ok(QueryResult::err("Agent ID cannot be empty".to_string()));
        }

        Ok(QueryResult::ok(
            AgentDto {
                id: query.agent_id,
                name: "Mock Agent".to_string(),
                status: "idle".to_string(),
                capabilities: vec!["coding".to_string()],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            false,
        ))
    }

    /// Handles ListAgents query.
    #[tracing::instrument(name = "handler.list_agents", skip(self))]
    pub async fn handle_list_agents(
        &self,
        query: ListAgents,
    ) -> Result<QueryResult<PaginatedResponse<AgentDto>>, ApplicationError> {
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0);

        let agents = vec![AgentDto {
            id: "agent_001".to_string(),
            name: "Codex".to_string(),
            status: "idle".to_string(),
            capabilities: vec!["coding".to_string(), "testing".to_string()],
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }];

        Ok(QueryResult::ok(
            PaginatedResponse::new(agents, 1, offset, limit),
            false,
        ))
    }
}

/// Task query handler.
pub struct TaskQueryHandler;

impl TaskQueryHandler {
    /// Handles GetTask query.
    #[tracing::instrument(name = "handler.get_task", skip(self))]
    pub async fn handle_get_task(
        &self,
        query: GetTask,
    ) -> Result<QueryResult<TaskDto>, ApplicationError> {
        Ok(QueryResult::ok(
            TaskDto {
                id: query.task_id,
                name: "Mock Task".to_string(),
                description: Some("A mock task".to_string()),
                status: "pending".to_string(),
                priority: "medium".to_string(),
                assigned_agent_id: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                completed_at: None,
            },
            false,
        ))
    }

    /// Handles ListTasks query.
    #[tracing::instrument(name = "handler.list_tasks", skip(self))]
    pub async fn handle_list_tasks(
        &self,
        query: ListTasks,
    ) -> Result<QueryResult<PaginatedResponse<TaskDto>>, ApplicationError> {
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0);

        let tasks = vec![TaskDto {
            id: "task_001".to_string(),
            name: "Implement feature X".to_string(),
            description: None,
            status: "pending".to_string(),
            priority: "high".to_string(),
            assigned_agent_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
        }];

        Ok(QueryResult::ok(
            PaginatedResponse::new(tasks, 1, offset, limit),
            false,
        ))
    }
}

#[cfg(test)]
mod tests {
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
}
