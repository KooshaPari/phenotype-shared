//! # Priority
//!
//! Execution priority level.

/// Execution priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /// Lowest priority.
    Low = 0,
    /// Normal priority.
    Normal = 1,
    /// High priority.
    High = 2,
    /// Critical priority.
    Critical = 3,
}

impl Priority {
    /// Returns the numeric value.
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Returns true if this priority preempts the other.
    pub fn preempts(self, other: Priority) -> bool {
        self > other
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Normal => write!(f, "Normal"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Normal);
        assert!(Priority::Normal > Priority::Low);
    }

    #[test]
    fn test_preempts() {
        assert!(Priority::High.preempts(Priority::Low));
        assert!(!Priority::Low.preempts(Priority::High));
    }
}
