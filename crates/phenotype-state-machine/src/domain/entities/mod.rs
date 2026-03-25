//! # Entities
//!
//! Core domain entities for the state machine.
//!
//! ## Main Entities
//!
//! - [`StateMachine<S>`] - Generic finite state machine with transition guards
//! - [`TransitionRecord<S>`] - Record of a state transition
//! - [`StateMachineConfig`] - Configuration for state machine behavior
//! - [`State`] - Trait for states in the machine

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Errors for state machine operations.
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    #[error("invalid transition from {from} to {to}: {reason}")]
    InvalidTransition {
        from: String,
        to: String,
        reason: String,
    },

    #[error("guard rejected transition from {from} to {to}: {reason}")]
    GuardRejected {
        from: String,
        to: String,
        reason: String,
    },
}

/// A state in a finite state machine. Implement this for your state enum.
pub trait State: fmt::Display + Clone + PartialEq + Eq + Serialize + serde::de::DeserializeOwned + 'static {
    /// Ordered index for determining valid forward transitions.
    /// Lower ordinals come before higher ordinals in the lifecycle.
    fn ordinal(&self) -> u32;

    /// All states in lifecycle order.
    fn all_states() -> Vec<Self>;
}

/// A guard that can approve or reject a transition.
pub trait TransitionGuard<S: State>: Send + Sync {
    /// Check whether the transition is allowed. Return `Ok(())` to allow,
    /// or `Err(reason)` to reject.
    fn check(&self, from: &S, to: &S) -> Result<(), String>;
}

/// Record of a state transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: State")]
pub struct TransitionRecord<S: State> {
    /// The state before the transition.
    pub from: S,
    /// The state after the transition.
    pub to: S,
    /// States that were skipped during this transition.
    pub skipped: Vec<S>,
    /// When the transition occurred.
    pub timestamp: DateTime<Utc>,
}

impl<S: State> fmt::Display for TransitionRecord<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

/// Configuration for the state machine.
#[derive(Debug, Clone)]
pub struct StateMachineConfig {
    /// Whether to allow skipping intermediate states (forward jumps).
    pub allow_skip: bool,
}

impl Default for StateMachineConfig {
    fn default() -> Self {
        Self { allow_skip: true }
    }
}

/// Generic finite state machine with forward-only enforcement and optional guards.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: State")]
pub struct StateMachine<S: State> {
    current: S,
    history: Vec<TransitionRecord<S>>,
}

impl<S: State> StateMachine<S> {
    /// Create a new state machine starting at the given state.
    pub fn new(initial: S) -> Self {
        Self {
            current: initial,
            history: Vec::new(),
        }
    }

    /// Create a new state machine from a persisted state.
    pub fn restore(current: S, history: Vec<TransitionRecord<S>>) -> Self {
        Self { current, history }
    }

    /// Get the current state.
    pub fn current(&self) -> &S {
        &self.current
    }

    /// Get the transition history.
    pub fn history(&self) -> &[TransitionRecord<S>] {
        &self.history
    }

    /// Attempt a transition to the target state (forward-only, no guards).
    pub fn transition(&mut self, target: S) -> Result<&TransitionRecord<S>, StateMachineError> {
        self.transition_with_config(target, &StateMachineConfig::default())
    }

    /// Attempt a transition with custom configuration.
    pub fn transition_with_config(
        &mut self,
        target: S,
        config: &StateMachineConfig,
    ) -> Result<&TransitionRecord<S>, StateMachineError> {
        // Enforce forward-only
        if target.ordinal() <= self.current.ordinal() {
            return Err(StateMachineError::InvalidTransition {
                from: self.current.to_string(),
                to: target.to_string(),
                reason: "backward transitions are not allowed".to_string(),
            });
        }

        // Check for skipped states
        let skipped: Vec<S> = S::all_states()
            .into_iter()
            .filter(|s| s.ordinal() > self.current.ordinal() && s.ordinal() < target.ordinal())
            .collect();

        if !config.allow_skip && !skipped.is_empty() {
            return Err(StateMachineError::InvalidTransition {
                from: self.current.to_string(),
                to: target.to_string(),
                reason: format!(
                    "skipping states is not allowed; would skip: {}",
                    skipped
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            });
        }

        let record = TransitionRecord {
            from: self.current.clone(),
            to: target.clone(),
            skipped,
            timestamp: Utc::now(),
        };

        self.current = target;
        self.history.push(record);
        Ok(self.history.last().unwrap())
    }

    /// Attempt a guarded transition. The guard is checked before the transition proceeds.
    pub fn guarded_transition(
        &mut self,
        target: S,
        guard: &dyn TransitionGuard<S>,
    ) -> Result<&TransitionRecord<S>, StateMachineError> {
        // Check guard first
        guard
            .check(&self.current, &target)
            .map_err(|reason| StateMachineError::GuardRejected {
                from: self.current.to_string(),
                to: target.to_string(),
                reason,
            })?;

        self.transition(target)
    }

    /// Check if a transition to the target state would be valid (forward-only check only).
    pub fn can_transition(&self, target: &S) -> bool {
        target.ordinal() > self.current.ordinal()
    }

    /// Get the list of reachable states from the current state.
    pub fn reachable_states(&self) -> Vec<S> {
        S::all_states()
            .into_iter()
            .filter(|s| s.ordinal() > self.current.ordinal())
            .collect()
    }

    /// Get the total number of transitions made.
    pub fn transition_count(&self) -> usize {
        self.history.len()
    }

    /// Check if the state machine is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.reachable_states().is_empty()
    }
}

impl<S: State> fmt::Display for StateMachine<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StateMachine({})", self.current)
    }
}
