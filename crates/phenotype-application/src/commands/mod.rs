//!
//! # Commands Module
//!
//! Commands represent **write operations** that modify state.
//! Following **CQRS**, commands are separated from queries.
//!
//! ## Naming Convention (SpecDD + BDD)
//!
//! Commands follow the pattern: `Verb+Noun` where:
//! - Verb is an imperative action (Create, Update, Delete, Assign, Start, Stop)
//! - Noun is the aggregate being acted upon
//!
//! ## Command Properties
//!
//! - **Intent-revealing**: Name reveals the intent, not implementation
//! - **Self-contained**: Commands contain all data needed to execute
//! - **Validated**: Commands are validated before processing
//! - **Auditable**: Commands produce domain events
//!
//! ## Examples
//!
//! ```rust,ignore
//! CreateAgent { name: "Codex", capabilities: vec!["coding"] }
//! UpdateAgentStatus { agent_id: "agent_xxx", status: AgentStatus::Busy }
//! AssignTask { task_id: "task_yyy", agent_id: "agent_xxx" }
//! CompleteTask { task_id: "task_yyy", result: "Done" }
//! ```

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Base command trait for CQRS pattern.
pub trait Command: Send + Sync {
    /// Returns the command type name for logging/tracing.
    fn command_type(&self) -> &'static str;
}

// =================================================================================================
// AGENT COMMANDS
// =================================================================================================

/// Command to create a new agent.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAgent {
    /// Human-readable name for the agent.
    pub name: String,

    /// Initial capabilities/tags.
    #[validate(length(min = 1))]
    pub capabilities: Vec<String>,
}

impl Command for CreateAgent {
    fn command_type(&self) -> &'static str {
        "CreateAgent"
    }
}

/// Command to update agent status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentStatus {
    /// Target agent ID.
    pub agent_id: String,

    /// New status.
    pub status: String,
}

impl Command for UpdateAgentStatus {
    fn command_type(&self) -> &'static str {
        "UpdateAgentStatus"
    }
}

/// Command to add a capability to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAgentCapability {
    /// Target agent ID.
    pub agent_id: String,

    /// Capability to add.
    pub capability: String,
}

impl Command for AddAgentCapability {
    fn command_type(&self) -> &'static str {
        "AddAgentCapability"
    }
}

/// Command to remove an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgent {
    /// Agent ID to delete.
    pub agent_id: String,

    /// Reason for deletion.
    pub reason: Option<String>,
}

impl Command for DeleteAgent {
    fn command_type(&self) -> &'static str {
        "DeleteAgent"
    }
}

// =================================================================================================
// TASK COMMANDS
// =================================================================================================

/// Command to create a new task.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTask {
    /// Human-readable task name.
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    /// Task description.
    pub description: Option<String>,

    /// Priority level.
    pub priority: String,

    /// Required capabilities.
    pub required_capabilities: Vec<String>,
}

impl Command for CreateTask {
    fn command_type(&self) -> &'static str {
        "CreateTask"
    }
}

/// Command to assign a task to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignTask {
    /// Task ID to assign.
    pub task_id: String,

    /// Agent ID to assign to.
    pub agent_id: String,
}

impl Command for AssignTask {
    fn command_type(&self) -> &'static str {
        "AssignTask"
    }
}

/// Command to start executing a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTask {
    /// Task ID to start.
    pub task_id: String,
}

impl Command for StartTask {
    fn command_type(&self) -> &'static str {
        "StartTask"
    }
}

/// Command to complete a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTask {
    /// Task ID to complete.
    pub task_id: String,

    /// Completion result.
    pub result: String,
}

impl Command for CompleteTask {
    fn command_type(&self) -> &'static str {
        "CompleteTask"
    }
}

/// Command to cancel a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTask {
    /// Task ID to cancel.
    pub task_id: String,

    /// Cancellation reason.
    pub reason: String,
}

impl Command for CancelTask {
    fn command_type(&self) -> &'static str {
        "CancelTask"
    }
}

// =================================================================================================
// WORKFLOW COMMANDS
// =================================================================================================

/// Command to start a workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StartWorkflow {
    /// Workflow name.
    #[validate(length(min = 1))]
    pub name: String,

    /// Input parameters.
    pub input: serde_json::Value,
}

impl Command for StartWorkflow {
    fn command_type(&self) -> &'static str {
        "StartWorkflow"
    }
}

/// Command to cancel a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelWorkflow {
    /// Workflow ID.
    pub workflow_id: String,

    /// Cancellation reason.
    pub reason: String,
}

impl Command for CancelWorkflow {
    fn command_type(&self) -> &'static str {
        "CancelWorkflow"
    }
}

// =================================================================================================
// BATCH COMMANDS
// =================================================================================================

/// Batch command to create multiple agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateAgents {
    /// Agents to create.
    pub agents: Vec<CreateAgent>,
}

impl Command for BatchCreateAgents {
    fn command_type(&self) -> &'static str {
        "BatchCreateAgents"
    }
}
