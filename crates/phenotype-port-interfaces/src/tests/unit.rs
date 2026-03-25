//! Unit tests for port interfaces.

use crate::prelude::*;
use chrono::Utc;

#[test]
fn test_string_id() {
    let id = StringId::new("test-123");
    assert_eq!(id.as_str(), "test-123");
    assert_eq!(id.to_string(), "test-123");
}

#[test]
fn test_string_id_from_uuid() {
    let id = StringId::from_uuid();
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_message_creation() {
    let msg = Message::new("hello".to_string())
        .with_correlation_id("corr-123")
        .with_header("key", "value");

    assert!(!msg.id.is_empty());
    assert_eq!(msg.correlation_id.as_deref(), Some("corr-123"));
    assert_eq!(msg.headers.get("key"), Some(&"value".to_string()));
}

#[test]
fn test_paginated() {
    let items = vec![1, 2, 3, 4, 5];
    let paginated = Paginated::new(items, 1, 2, 5);

    assert_eq!(paginated.total_pages(), 3);
    assert!(paginated.has_next());
    assert!(!paginated.has_prev());
}

#[test]
fn test_log_level() {
    assert_eq!(LogLevel::from_str("debug"), LogLevel::Debug);
    assert_eq!(LogLevel::from_str("WARN"), LogLevel::Warn);
    assert_eq!(LogLevel::from_str("invalid"), LogLevel::Info);
}
