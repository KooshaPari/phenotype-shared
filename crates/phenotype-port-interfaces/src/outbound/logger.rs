//! # Logger Ports
//!
//! Logger ports define logging operations.

use crate::error::Result;
use std::fmt::Debug;

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

/// Log record.
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub level: LogLevel,
    pub message: String,
    pub target: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub fields: std::collections::HashMap<String, serde_json::Value>,
}

impl LogRecord {
    pub fn new(level: LogLevel, target: &str, message: String) -> Self {
        Self {
            level,
            target: target.to_string(),
            message,
            timestamp: chrono::Utc::now(),
            fields: std::collections::HashMap::new(),
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Logger port for logging operations.
pub trait Logger: Send + Sync {
    /// Log a record.
    fn log(&self, record: &LogRecord) -> Result<()>;

    /// Check if a level is enabled.
    fn is_enabled(&self, level: LogLevel) -> bool;

    /// Create a child logger with additional context.
    fn with_target(&self, target: &str) -> Box<dyn Logger>;
}

/// Extension trait for logger with convenience methods.
pub trait LoggerExt: Logger {
    fn trace(&self, target: &str, message: &str) -> Result<()> {
        self.log(&LogRecord::new(LogLevel::Trace, target, message.to_string()))
    }

    fn debug(&self, target: &str, message: &str) -> Result<()> {
        self.log(&LogRecord::new(LogLevel::Debug, target, message.to_string()))
    }

    fn info(&self, target: &str, message: &str) -> Result<()> {
        self.log(&LogRecord::new(LogLevel::Info, target, message.to_string()))
    }

    fn warn(&self, target: &str, message: &str) -> Result<()> {
        self.log(&LogRecord::new(LogLevel::Warn, target, message.to_string()))
    }

    fn error(&self, target: &str, message: &str) -> Result<()> {
        self.log(&LogRecord::new(LogLevel::Error, target, message.to_string()))
    }

    fn log_structured(&self, level: LogLevel, target: &str, message: &str, fields: impl IntoIterator<Item = (String, serde_json::Value)>) -> Result<()> {
        let mut record = LogRecord::new(level, target, message.to_string());
        for (k, v) in fields {
            record.fields.insert(k, v);
        }
        self.log(&record)
    }
}

impl<T: Logger> LoggerExt for T {}
