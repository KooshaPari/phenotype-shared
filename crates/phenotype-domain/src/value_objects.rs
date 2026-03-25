//! # Value Objects
//!
//! Value objects are immutable, interchangeable by identity.
//! They have no conceptual identity - two value objects with same values are equal.
//!
//! ## Principles (xDD stack)
//!
//! - **Immutability**: Once created, cannot be modified
//! - **Valid construction**: Validation in constructor
//! - **Self-contained**: No external dependencies
//! - **Testability**: Easy to test in isolation
//! - **Equality by value**: Two VOs with same values are equal

pub mod agent_id;
pub mod agent_name;
pub mod agent_status;
pub mod priority;
pub mod task_id;
pub mod task_name;
pub mod task_status;
pub mod timestamp;
pub mod workflow_id;
pub mod workflow_name;
pub mod policy_id;

// Re-exports
pub use agent_id::AgentId;
pub use agent_name::AgentName;
pub use agent_status::AgentStatus;
pub use priority::Priority;
pub use task_id::TaskId;
pub use task_name::TaskName;
pub use task_status::TaskStatus;
pub use timestamp::Timestamp;
pub use workflow_id::WorkflowId;
pub use workflow_name::WorkflowName;
pub use policy_id::PolicyId;
