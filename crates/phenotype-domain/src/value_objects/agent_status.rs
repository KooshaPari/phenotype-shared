//! # Agent Status

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
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Idle => write!(f, "Idle"),
            AgentStatus::Active => write!(f, "Active"),
            AgentStatus::Busy => write!(f, "Busy"),
            AgentStatus::Paused => write!(f, "Paused"),
            AgentStatus::Stopped => write!(f, "Stopped"),
            AgentStatus::Error => write!(f, "Error"),
        }
    }
}

impl std::str::FromStr for AgentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "idle" => Ok(AgentStatus::Idle),
            "active" => Ok(AgentStatus::Active),
            "busy" => Ok(AgentStatus::Busy),
            "paused" => Ok(AgentStatus::Paused),
            "stopped" => Ok(AgentStatus::Stopped),
            "error" => Ok(AgentStatus::Error),
            _ => Err(format!("unknown status: {s}")),
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
        assert!(AgentStatus::Stopped.is_terminal());
        assert!(AgentStatus::Error.is_terminal());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("idle".parse::<AgentStatus>().unwrap(), AgentStatus::Idle);
        assert_eq!("ACTIVE".parse::<AgentStatus>().unwrap(), AgentStatus::Active);
        assert!("unknown".parse::<AgentStatus>().is_err());
    }
}
