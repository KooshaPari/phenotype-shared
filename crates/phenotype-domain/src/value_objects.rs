//! # Value Objects
//!
//! Value Objects are immutable objects defined by their attributes.
//!
//! ## Design Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | **Immutability** | Never modify after creation |
//! | **Value Equality** | Equal by attribute, not identity |
//! | **Self-Validation** | Validated at construction |
//!
//! ## Examples
//!
//! ```rust
//! use phenotype_domain::value_objects::{AgentId, Priority};
//!
//! let id = AgentId::new("agent-001")?;
//! let priority = Priority::High;
//! ```

pub mod agent_id;
pub mod agent_name;
pub mod agent_status;
pub mod task_id;
pub mod task_name;
pub mod task_status;
pub mod workflow_id;
pub mod workflow_name;
pub mod policy_id;
pub mod priority;
pub mod timestamp;

pub use agent_id::AgentId;
pub use agent_name::AgentName;
pub use agent_status::AgentStatus;
pub use task_id::TaskId;
pub use task_name::TaskName;
pub use task_status::TaskStatus;
pub use workflow_id::WorkflowId;
pub use workflow_name::WorkflowName;
pub use policy_id::PolicyId;
pub use priority::Priority;
pub use timestamp::Timestamp;
