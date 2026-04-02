use crate::commands::*;
use crate::dto::*;
use crate::queries::*;
use crate::{ApplicationError, CommandResult, QueryResult};

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
            return Ok(CommandResult::err(
                "Task ID and Agent ID are required".to_string(),
            ));
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
