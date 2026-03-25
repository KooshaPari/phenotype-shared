//! # Unit Tests for Domain Entities

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum TestState {
    Created,
    InProgress,
    Review,
    Done,
}

impl fmt::Display for TestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Review => write!(f, "review"),
            Self::Done => write!(f, "done"),
        }
    }
}

impl State for TestState {
    fn ordinal(&self) -> u32 {
        match self {
            Self::Created => 0,
            Self::InProgress => 1,
            Self::Review => 2,
            Self::Done => 3,
        }
    }

    fn all_states() -> Vec<Self> {
        vec![
            Self::Created,
            Self::InProgress,
            Self::Review,
            Self::Done,
        ]
    }
}

#[test]
fn forward_transition() {
    let mut sm = StateMachine::new(TestState::Created);
    let record = sm.transition(TestState::InProgress).unwrap();
    assert_eq!(record.from, TestState::Created);
    assert_eq!(record.to, TestState::InProgress);
    assert!(record.skipped.is_empty());
    assert_eq!(*sm.current(), TestState::InProgress);
}

#[test]
fn backward_transition_rejected() {
    let mut sm = StateMachine::new(TestState::Review);
    let err = sm.transition(TestState::Created).unwrap_err();
    match err {
        StateMachineError::InvalidTransition { from, to, .. } => {
            assert_eq!(from, "review");
            assert_eq!(to, "created");
        }
        _ => panic!("expected InvalidTransition"),
    }
}

#[test]
fn same_state_transition_rejected() {
    let mut sm = StateMachine::new(TestState::InProgress);
    assert!(sm.transition(TestState::InProgress).is_err());
}

#[test]
fn skip_states_allowed_by_default() {
    let mut sm = StateMachine::new(TestState::Created);
    let record = sm.transition(TestState::Done).unwrap();
    assert_eq!(record.skipped.len(), 2);
    assert!(record.skipped.contains(&TestState::InProgress));
    assert!(record.skipped.contains(&TestState::Review));
}

#[test]
fn skip_states_rejected_when_disallowed() {
    let mut sm = StateMachine::new(TestState::Created);
    let config = StateMachineConfig { allow_skip: false };
    let err = sm
        .transition_with_config(TestState::Done, &config)
        .unwrap_err();
    match err {
        StateMachineError::InvalidTransition { reason, .. } => {
            assert!(reason.contains("skipping"));
        }
        _ => panic!("expected InvalidTransition"),
    }
}

#[test]
fn guard_allows_transition() {
    struct AllowAll;
    impl TransitionGuard<TestState> for AllowAll {
        fn check(&self, _from: &TestState, _to: &TestState) -> Result<(), String> {
            Ok(())
        }
    }

    let mut sm = StateMachine::new(TestState::Created);
    sm.guarded_transition(TestState::InProgress, &AllowAll)
        .unwrap();
    assert_eq!(*sm.current(), TestState::InProgress);
}

#[test]
fn guard_rejects_transition() {
    struct DenyAll;
    impl TransitionGuard<TestState> for DenyAll {
        fn check(&self, _from: &TestState, _to: &TestState) -> Result<(), String> {
            Err("not allowed".to_string())
        }
    }

    let mut sm = StateMachine::new(TestState::Created);
    let err = sm
        .guarded_transition(TestState::InProgress, &DenyAll)
        .unwrap_err();
    match err {
        StateMachineError::GuardRejected { reason, .. } => {
            assert_eq!(reason, "not allowed");
        }
        _ => panic!("expected GuardRejected"),
    }
}

#[test]
fn can_transition_check() {
    let sm = StateMachine::new(TestState::InProgress);
    assert!(sm.can_transition(&TestState::Review));
    assert!(sm.can_transition(&TestState::Done));
    assert!(!sm.can_transition(&TestState::Created));
    assert!(!sm.can_transition(&TestState::InProgress));
}

#[test]
fn reachable_states() {
    let sm = StateMachine::new(TestState::InProgress);
    let reachable = sm.reachable_states();
    assert_eq!(reachable, vec![TestState::Review, TestState::Done]);
}

#[test]
fn history_tracking() {
    let mut sm = StateMachine::new(TestState::Created);
    sm.transition(TestState::InProgress).unwrap();
    sm.transition(TestState::Review).unwrap();
    assert_eq!(sm.history().len(), 2);
}

#[test]
fn display_impl() {
    let sm = StateMachine::new(TestState::Created);
    assert_eq!(format!("{}", sm), "StateMachine(created)");
}

#[test]
fn transition_count() {
    let mut sm = StateMachine::new(TestState::Created);
    assert_eq!(sm.transition_count(), 0);
    sm.transition(TestState::InProgress).unwrap();
    assert_eq!(sm.transition_count(), 1);
    sm.transition(TestState::Review).unwrap();
    assert_eq!(sm.transition_count(), 2);
}

#[test]
fn is_terminal_when_no_reachable_states() {
    let mut sm = StateMachine::new(TestState::Created);
    assert!(!sm.is_terminal());
    sm.transition(TestState::Done).unwrap();
    assert!(sm.is_terminal());
}

#[test]
fn restore_from_history() {
    let history = vec![
        TransitionRecord {
            from: TestState::Created,
            to: TestState::InProgress,
            skipped: vec![],
            timestamp: chrono::Utc::now(),
        },
        TransitionRecord {
            from: TestState::InProgress,
            to: TestState::Review,
            skipped: vec![],
            timestamp: chrono::Utc::now(),
        },
    ];
    let sm = StateMachine::<TestState>::restore(TestState::Review, history);
    assert_eq!(*sm.current(), TestState::Review);
    assert_eq!(sm.transition_count(), 2);
}
