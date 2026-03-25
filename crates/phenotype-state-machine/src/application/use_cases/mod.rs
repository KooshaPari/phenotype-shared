//! # Application Use Cases
//!
//! Use cases orchestrate the domain logic and coordinate between ports.

use crate::domain::entities::{StateMachine, StateMachineError, State, TransitionRecord};
use crate::domain::ports::outbound::{StateMachineRepository, RepositoryError};
use crate::application::dto::{StateMachineDto, TransitionRecordDto};

/// Error type for use case operations.
#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("state machine not found: {0}")]
    NotFound(String),

    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("state machine error: {0}")]
    StateMachine(#[from] StateMachineError),

    #[error("invalid state: {0}")]
    InvalidState(String),
}

/// Use case for creating a new state machine.
pub struct CreateStateMachineUseCase<S: State, R: StateMachineRepository<S>> {
    repository: R,
}

impl<S: State, R: StateMachineRepository<S>> CreateStateMachineUseCase<S, R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: &str, initial_state: S) -> Result<StateMachine<S>, UseCaseError> {
        let sm = StateMachine::new(initial_state);
        self.repository.save(id, &sm)?;
        Ok(sm)
    }
}

/// Use case for transitioning a state machine.
pub struct TransitionUseCase<S: State, R: StateMachineRepository<S>> {
    repository: R,
}

impl<S: State, R: StateMachineRepository<S>> TransitionUseCase<S, R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: &str, target_state: S) -> Result<TransitionRecord<S>, UseCaseError> {
        let mut sm = self.repository.find_by_id(id)?
            .ok_or_else(|| UseCaseError::NotFound(id.to_string()))?;

        let record = sm.transition(target_state)?;
        self.repository.save(id, &sm)?;

        Ok(record)
    }
}

/// Use case for getting state machine status.
pub struct GetStateMachineUseCase<S: State, R: StateMachineRepository<S>> {
    repository: R,
}

impl<S: State + std::fmt::Display, R: StateMachineRepository<S>> GetStateMachineUseCase<S, R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: &str) -> Result<Option<StateMachineDto>, UseCaseError> {
        let sm = self.repository.find_by_id(id)?;
        Ok(sm.map(|sm| StateMachineDto {
            id: id.to_string(),
            current_state: sm.current().to_string(),
            history: sm.history().iter().map(|r| TransitionRecordDto {
                from: r.from.to_string(),
                to: r.to.to_string(),
                skipped: r.skipped.iter().map(|s| s.to_string()).collect(),
                timestamp: r.timestamp.to_rfc3339(),
            }).collect(),
            is_terminal: sm.is_terminal(),
        }))
    }
}
