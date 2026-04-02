use crate::commands::*;
use crate::dto::*;
use crate::queries::*;
use crate::{ApplicationError, CommandResult, QueryResult};

/// Agent command handler.
/// Coordinates agent-related commands with domain and ports.
pub struct AgentCommandHandler {
    // In the full implementation, this would hold references to ports.
    // For now, we define the interface.
}

impl AgentCommandHandler {
    /// Handles CreateAgent command.
    #[tracing::instrument(name = "handler.create_agent", skip(self))]
    pub async fn handle_create_agent(
        &self,
        cmd: CreateAgent,
    ) -> Result<CommandResult<AgentDto>, ApplicationError> {
        if cmd.name.trim().is_empty() {
            return Ok(CommandResult::err("Name cannot be empty".to_string()));
        }

        let dto = AgentDto {
            id: format!("agent_{}", uuid::Uuid::new_v4()),
            name: cmd.name,
            status: "idle".to_string(),
            capabilities: cmd.capabilities,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(CommandResult::ok(dto, None))
    }

    /// Handles UpdateAgentStatus command.
    #[tracing::instrument(name = "handler.update_agent_status", skip(self))]
    pub async fn handle_update_agent_status(
        &self,
        cmd: UpdateAgentStatus,
    ) -> Result<CommandResult<AgentDto>, ApplicationError> {
        if cmd.agent_id.trim().is_empty() {
            return Ok(CommandResult::err("Agent ID cannot be empty".to_string()));
        }

        let valid_statuses = ["idle", "active", "busy", "paused", "stopped", "error"];
        if !valid_statuses.contains(&cmd.status.as_str()) {
            return Ok(CommandResult::err(format!(
                "Invalid status: {}",
                cmd.status
            )));
        }

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
