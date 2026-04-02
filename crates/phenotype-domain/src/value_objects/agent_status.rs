//! # Agent Status
//!
//! Status state machine for agent lifecycle.

/// Status of an agent in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentStatus {
    /// Agent is inactive and not processing tasks.
    #[default]
    Idle,
    /// Agent is active and available for tasks.
    Active,
    /// Agent is currently processing a task.
    Busy,
    /// Agent is paused by user or policy.
    Paused,
    /// Agent has been gracefully shut down.
    Stopped,
    /// Agent encountered an error.
    Error,
}

impl AgentStatus {
    /// Returns true if the agent can accept new tasks.
    pub fn can_accept_task(&self) -> bool {
        matches!(self, AgentStatus::Idle | AgentStatus::Active)
    }

    /// Returns true if the agent is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentStatus::Stopped | AgentStatus::Error)
    }

    /// Returns true if the agent is operational (not stopped/error).
    pub fn is_operational(&self) -> bool {
        !self.is_terminal()
    }

    /// Returns a short string code for the status.
    pub fn code(&self) -> &'static str {
        match self {
            AgentStatus::Idle => "IDLE",
            AgentStatus::Active => "ACTIVE",
            AgentStatus::Busy => "BUSY",
            AgentStatus::Paused => "PAUSED",
            AgentStatus::Stopped => "STOPPED",
            AgentStatus::Error => "ERROR",
        }
    }
}

impl core::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl core::str::FromStr for AgentStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "IDLE" => Ok(AgentStatus::Idle),
            "ACTIVE" => Ok(AgentStatus::Active),
            "BUSY" => Ok(AgentStatus::Busy),
            "PAUSED" => Ok(AgentStatus::Paused),
            "STOPPED" => Ok(AgentStatus::Stopped),
            "ERROR" => Ok(AgentStatus::Error),
            _ => Err("unknown agent status"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_accept_task() {
        assert!(AgentStatus::Idle.can_accept_task());
        assert!(AgentStatus::Active.can_accept_task());
        assert!(!AgentStatus::Busy.can_accept_task());
        assert!(!AgentStatus::Paused.can_accept_task());
        assert!(!AgentStatus::Stopped.can_accept_task());
        assert!(!AgentStatus::Error.can_accept_task());
    }

    #[test]
    fn test_is_terminal() {
        assert!(!AgentStatus::Idle.is_terminal());
        assert!(!AgentStatus::Active.is_terminal());
        assert!(!AgentStatus::Busy.is_terminal());
        assert!(!AgentStatus::Paused.is_terminal());
        assert!(AgentStatus::Stopped.is_terminal());
        assert!(AgentStatus::Error.is_terminal());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("idle".parse::<AgentStatus>().unwrap(), AgentStatus::Idle);
        assert_eq!(
            "ACTIVE".parse::<AgentStatus>().unwrap(),
            AgentStatus::Active
        );
        assert_eq!("busy".parse::<AgentStatus>().unwrap(), AgentStatus::Busy);
        assert!("unknown".parse::<AgentStatus>().is_err());
    }

    #[test]
    fn test_code() {
        assert_eq!(AgentStatus::Idle.code(), "IDLE");
        assert_eq!(AgentStatus::Active.code(), "ACTIVE");
    }
}
