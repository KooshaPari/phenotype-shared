//! # Data Transfer Objects
//!
//! DTOs are plain data structures used for transferring data between layers.

use serde::{Deserialize, Serialize};

/// Command to transition a state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionCommand {
    pub id: String,
    pub target_state: String,
    pub skip_validation: bool,
}

/// DTO for state machine state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineDto {
    pub id: String,
    pub current_state: String,
    pub history: Vec<TransitionRecordDto>,
    pub is_terminal: bool,
}

/// DTO for a transition record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRecordDto {
    pub from: String,
    pub to: String,
    pub skipped: Vec<String>,
    pub timestamp: String,
}
