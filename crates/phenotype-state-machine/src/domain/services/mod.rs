//! # Domain Services
//!
//! Domain services for state machine operations that don't belong to entities.

use super::entities::*;
use std::fmt::Debug;

/// Service for validating state transitions.
pub struct TransitionValidator<S: State> {
    config: StateMachineConfig,
}

impl<S: State> TransitionValidator<S> {
    pub fn new(config: StateMachineConfig) -> Self {
        Self { config }
    }

    /// Validate if a transition is possible without performing it.
    pub fn validate(&self, current: &S, target: &S) -> Result<ValidationResult, StateMachineError> {
        // Check forward-only rule
        if target.ordinal() <= current.ordinal() {
            return Err(StateMachineError::InvalidTransition {
                from: current.to_string(),
                to: target.to_string(),
                reason: "backward transitions are not allowed".to_string(),
            });
        }

        // Check skipped states
        let skipped: Vec<S> = S::all_states()
            .into_iter()
            .filter(|s| s.ordinal() > current.ordinal() && s.ordinal() < target.ordinal())
            .collect();

        if !self.config.allow_skip && !skipped.is_empty() {
            return Err(StateMachineError::InvalidTransition {
                from: current.to_string(),
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

        Ok(ValidationResult {
            allowed: true,
            skipped,
            will_skip: !skipped.is_empty(),
        })
    }

    /// Get all valid next states from the current state.
    pub fn valid_next_states(&self, current: &S) -> Vec<S> {
        let all_states = S::all_states();
        all_states
            .into_iter()
            .filter(|s| s.ordinal() > current.ordinal())
            .collect()
    }
}

/// Result of a validation check.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub allowed: bool,
    pub skipped: Vec<S>,
    pub will_skip: bool,
}

/// Guard that always allows transitions.
#[derive(Debug, Clone, Default)]
pub struct AllowAllGuard;

impl<S: State> TransitionGuard<S> for AllowAllGuard {
    fn check(&self, _from: &S, _to: &S) -> Result<(), String> {
        Ok(())
    }
}

/// Guard that never allows transitions.
#[derive(Debug, Clone, Default)]
pub struct DenyAllGuard;

impl<S: State> TransitionGuard<S> for DenyAllGuard {
    fn check(&self, _from: &S, _to: &S) -> Result<(), String> {
        Err("transitions are denied".to_string())
    }
}

/// Guard that allows transitions to specific states only.
#[derive(Debug, Clone)]
pub struct AllowStatesGuard<S: State> {
    allowed_targets: Vec<S>,
}

impl<S: State> AllowStatesGuard<S> {
    pub fn new(targets: impl IntoIterator<Item = S>) -> Self {
        Self {
            allowed_targets: targets.into_iter().collect(),
        }
    }
}

impl<S: State + PartialEq> TransitionGuard<S> for AllowStatesGuard<S> {
    fn check(&self, _from: &S, to: &S) -> Result<(), String> {
        if self.allowed_targets.contains(to) {
            Ok(())
        } else {
            Err(format!("state {} is not allowed", to))
        }
    }
}
