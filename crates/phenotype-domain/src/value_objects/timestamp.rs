//! # Timestamp
//!
//! Domain timestamp value object for event sourcing.
//! Uses Unix timestamp for simplicity and portability.

/// Timestamp in milliseconds since Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Creates a new timestamp for the current time.
    pub fn now() -> Self {
        let ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        Self(ms)
    }

    /// Creates a timestamp from milliseconds since Unix epoch.
    pub fn from_millis(ms: u64) -> Self {
        Self(ms)
    }

    /// Returns the timestamp as milliseconds since Unix epoch.
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    /// Returns true if this timestamp is in the past.
    pub fn is_past(&self) -> bool {
        self.0 < Self::now().0
    }

    /// Returns true if this timestamp is in the future.
    pub fn is_future(&self) -> bool {
        self.0 > Self::now().0
    }

    /// Returns a human-readable ISO 8601-like string (simplified).
    pub fn to_iso_string(&self) -> String {
        let secs = self.0 / 1000;
        let ms = self.0 % 1000;
        format!("{}ms.{}", secs, ms)
    }

    /// Adds a duration in milliseconds.
    pub fn add_ms(&self, ms: u64) -> Self {
        Self(self.0.saturating_add(ms))
    }

    /// Returns the difference in milliseconds to another timestamp.
    pub fn diff_ms(&self, other: &Timestamp) -> i64 {
        self.0 as i64 - other.0 as i64
    }
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_iso_string())
    }
}

impl From<Timestamp> for u64 {
    fn from(t: Timestamp) -> Self {
        t.0
    }
}

impl From<u64> for Timestamp {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let ts = Timestamp::now();
        assert!(ts.as_millis() > 0);
    }

    #[test]
    fn test_is_past_future() {
        let past = Timestamp::from_millis(0);
        let future = Timestamp::from_millis(u64::MAX);
        assert!(past.is_past());
        assert!(future.is_future());
    }

    #[test]
    fn test_add_ms() {
        let ts = Timestamp::from_millis(1000);
        let added = ts.add_ms(500);
        assert_eq!(added.as_millis(), 1500);
    }

    #[test]
    fn test_diff() {
        let ts1 = Timestamp::from_millis(1000);
        let ts2 = Timestamp::from_millis(1500);
        assert_eq!(ts1.diff_ms(&ts2), -500);
        assert_eq!(ts2.diff_ms(&ts1), 500);
    }

    #[test]
    fn test_ordering() {
        let t1 = Timestamp::from_millis(1000);
        let t2 = Timestamp::from_millis(2000);
        assert!(t1 < t2);
        assert!(t1 <= t2);
        assert!(t2 > t1);
    }
}
