//! # Priority
//!
//! Priority levels for tasks and workflows.
//! Lower values = higher priority.

/// Priority level (0 = highest priority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Priority(u8);

impl Priority {
    /// Priority 0: Critical/Highest.
    pub const CRITICAL: Priority = Priority(0);
    /// Priority 1: High.
    pub const HIGH: Priority = Priority(1);
    /// Priority 2: Normal (default).
    pub const NORMAL: Priority = Priority(2);
    /// Priority 3: Low.
    pub const LOW: Priority = Priority(3);
    /// Priority 4: Background/Lowest.
    pub const BACKGROUND: Priority = Priority(4);

    /// Creates a new priority from a numeric value (clamped to `u8::MAX`).
    pub fn new(value: u32) -> Self {
        Self(value.min(u8::MAX as u32) as u8)
    }

    /// Returns the raw priority value.
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Returns true if this is a realtime priority (0 or 1).
    pub fn is_realtime(&self) -> bool {
        self.0 <= 1
    }

    /// Returns true if this priority is higher than another.
    pub fn is_higher_than(&self, other: &Priority) -> bool {
        self.0 < other.0
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self.0 {
            0 => "CRITICAL",
            1 => "HIGH",
            2 => "NORMAL",
            3 => "LOW",
            _ => "BACKGROUND",
        }
    }
}

impl core::fmt::Display for Priority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.label(), self.0)
    }
}

impl From<Priority> for u8 {
    fn from(p: Priority) -> Self {
        p.0
    }
}

impl From<u8> for Priority {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::CRITICAL < Priority::HIGH);
        assert!(Priority::HIGH < Priority::NORMAL);
        assert!(Priority::NORMAL < Priority::LOW);
        assert!(Priority::LOW < Priority::BACKGROUND);
    }

    #[test]
    fn test_is_realtime() {
        assert!(Priority::CRITICAL.is_realtime());
        assert!(Priority::HIGH.is_realtime());
        assert!(!Priority::NORMAL.is_realtime());
        assert!(!Priority::LOW.is_realtime());
        assert!(!Priority::BACKGROUND.is_realtime());
    }

    #[test]
    fn test_is_higher_than() {
        assert!(Priority::HIGH.is_higher_than(&Priority::NORMAL));
        assert!(!Priority::NORMAL.is_higher_than(&Priority::HIGH));
    }

    #[test]
    fn test_clamping() {
        // Test at boundary - priority should clamp to 255
        let p = Priority::new(255);
        assert_eq!(p.value(), 255);

        // Test below range - should be 0
        let p = Priority::new(0);
        assert_eq!(p.value(), 0);
    }

    #[test]
    fn test_labels() {
        assert_eq!(Priority::CRITICAL.label(), "CRITICAL");
        assert_eq!(Priority::NORMAL.label(), "NORMAL");
        assert_eq!(Priority::BACKGROUND.label(), "BACKGROUND");
    }
}
