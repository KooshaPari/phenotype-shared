//! Core configuration types and traits for Phenotype crates.
//!
//! This crate provides shared configuration abstractions used across
//! the Phenotype ecosystem.

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::Path;

/// Configuration source priority (higher = takes precedence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Priority(u8);

impl Priority {
    /// Lowest priority (default).
    pub const LOWEST: u8 = 0;
    /// Default configuration priority.
    pub const DEFAULT: u8 = 50;
    /// Environment variable priority.
    pub const ENV: u8 = 75;
    /// Highest priority (user overrides).
    pub const HIGHEST: u8 = 100;

    /// Creates a new priority with the given value.
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the raw priority value.
    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Priority {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Priority> for u8 {
    fn from(p: Priority) -> Self {
        p.0
    }
}

/// Configuration source that can provide values.
#[derive(Debug, Clone)]
pub struct ConfigSource {
    /// Source name (e.g., "env", "file", "default").
    pub name: String,
    /// Priority of this source.
    pub priority: Priority,
}

impl ConfigSource {
    /// Creates a new configuration source.
    pub fn new(name: impl Into<String>, priority: Priority) -> Self {
        Self { name: name.into(), priority }
    }

    /// Creates a source with default priority.
    pub fn default_source(name: impl Into<String>) -> Self {
        Self::new(name, Priority::new(Priority::DEFAULT))
    }
}

/// Trait for configuration loaders.
pub trait ConfigLoader: Send + Sync {
    /// Loads configuration from this source as a JSON value.
    fn load_value(&self) -> Result<Value, Box<dyn std::error::Error>>;

    /// Returns the source name.
    fn source_name(&self) -> &str;

    /// Loads configuration from this source into a typed structure.
    fn load<T: DeserializeOwned>(&self) -> Result<T, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        serde_json::from_value(self.load_value()?)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

/// Trait for configuration validation.
pub trait ValidateConfig {
    /// Validates the configuration.
    fn validate(&self) -> Result<(), ConfigValidationError>;
}

/// Configuration validation error.
#[derive(Debug, thiserror::Error)]
#[error("configuration validation failed: {0}")]
pub struct ConfigValidationError(pub String);

impl ConfigValidationError {
    /// Creates a new validation error.
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

/// Environment-based configuration source.
#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    prefix: Option<String>,
}

impl EnvConfig {
    /// Creates a new environment configuration source.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new environment configuration source with a prefix.
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self { prefix: Some(prefix.into()) }
    }

    /// Gets a value from the environment.
    pub fn get(&self, key: &str) -> Option<String> {
        let key = match &self.prefix {
            Some(p) => format!("{}_{}", p, key),
            None => key.to_uppercase(),
        };
        std::env::var(key).ok()
    }
}

impl ConfigLoader for EnvConfig {
    fn load_value(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let env_vars: serde_json::Map<String, Value> = match &self.prefix {
            Some(prefix) => {
                let prefix = format!("{prefix}_");
                std::env::vars()
                    .filter_map(|(key, value)| {
                        key.strip_prefix(&prefix)
                            .map(|stripped| (stripped.to_uppercase(), env_value(value)))
                    })
                    .collect()
            }
            None => std::env::vars()
                .map(|(key, value)| (key.to_uppercase(), env_value(value)))
                .collect(),
        };

        Ok(Value::Object(env_vars))
    }

    fn source_name(&self) -> &str {
        "environment"
    }
}

/// File-based configuration source.
#[derive(Debug, Clone)]
pub struct FileConfig {
    path: std::path::PathBuf,
    format: ConfigFormat,
}

/// Supported configuration file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfigFormat {
    #[default]
    Json,
    Toml,
    Yaml,
}

impl ConfigFormat {
    /// Detects format from file extension.
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}

impl FileConfig {
    /// Creates a new file configuration source.
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        let path = path.into();
        let format = ConfigFormat::from_path(&path).unwrap_or(ConfigFormat::Json);
        Self { path, format }
    }

    /// Loads configuration from the file.
    pub fn load<T: DeserializeOwned>(&self) -> Result<T, FileConfigError> {
        let content = std::fs::read_to_string(&self.path)?;

        match self.format {
            ConfigFormat::Json => serde_json::from_str(&content).map_err(Into::into),
            ConfigFormat::Toml => toml::from_str(&content).map_err(Into::into),
            ConfigFormat::Yaml => serde_yaml::from_str(&content).map_err(Into::into),
        }
    }
}

impl ConfigLoader for FileConfig {
    fn load_value(&self) -> Result<Value, Box<dyn std::error::Error>> {
        self.load::<Value>().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn source_name(&self) -> &str {
        self.path.to_str().unwrap_or("file")
    }
}

/// File configuration error.
#[derive(Debug, thiserror::Error)]
pub enum FileConfigError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("failed to parse YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// Configuration merge error.
#[derive(Debug, thiserror::Error)]
pub enum ConfigMergeError {
    #[error("configuration source `{source_name}` returned {kind}; expected object")]
    NonObject { source_name: String, kind: &'static str },
}

/// Merges multiple configuration sources.
pub fn merge_configs<T: DeserializeOwned>(
    sources: &[&dyn ConfigLoader],
) -> Result<T, Box<dyn std::error::Error>> {
    let mut merged = serde_json::Map::new();

    for source in sources {
        match source.load_value() {
            Ok(Value::Object(map)) => {
                merge_objects(&mut merged, map);
            }
            Ok(value) => {
                return Err(Box::new(ConfigMergeError::NonObject {
                    source_name: source.source_name().to_string(),
                    kind: value_kind(&value),
                }));
            }
            Err(err) => return Err(err),
        }
    }

    serde_json::from_value(Value::Object(merged))
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn merge_objects(
    target: &mut serde_json::Map<String, Value>,
    source: serde_json::Map<String, Value>,
) {
    for (key, source_value) in source {
        match (target.get_mut(&key), source_value) {
            (Some(Value::Object(target_object)), Value::Object(source_object)) => {
                merge_objects(target_object, source_object);
            }
            (_, value) => {
                target.insert(key, value);
            }
        }
    }
}

fn env_value(value: String) -> Value {
    serde_json::from_str(&value).unwrap_or(Value::String(value))
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticLoader {
        name: &'static str,
        value: Value,
    }

    impl ConfigLoader for StaticLoader {
        fn load_value(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.value.clone())
        }

        fn source_name(&self) -> &str {
            self.name
        }
    }

    struct ErrorLoader;

    impl ConfigLoader for ErrorLoader {
        fn load_value(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid config")))
        }

        fn source_name(&self) -> &str {
            "error-source"
        }
    }

    #[test]
    fn test_priority_ordering() {
        let low = Priority::LOWEST;
        let high = Priority::HIGHEST;
        assert!(low < high);
    }

    #[test]
    fn test_env_config() {
        std::env::set_var("TEST_VAR", "test_value");
        let config = EnvConfig::new();
        assert_eq!(config.get("TEST_VAR"), Some("test_value".to_string()));
        std::env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_prefixed_env_config_load_value_is_scoped() {
        std::env::set_var("APP_ALLOWED", "scoped");
        std::env::set_var("APP_PORT", "8080");
        std::env::set_var("APP_ENABLED", "true");
        std::env::set_var("OTHER_ALLOWED", "global");

        let value = EnvConfig::with_prefix("APP").load_value().unwrap();

        assert_eq!(value.get("ALLOWED"), Some(&Value::String("scoped".to_string())));
        assert_eq!(value.get("PORT"), Some(&serde_json::json!(8080)));
        assert_eq!(value.get("ENABLED"), Some(&serde_json::json!(true)));
        assert!(value.get("APP_ALLOWED").is_none());
        assert!(value.get("OTHER_ALLOWED").is_none());

        std::env::remove_var("APP_ALLOWED");
        std::env::remove_var("APP_PORT");
        std::env::remove_var("APP_ENABLED");
        std::env::remove_var("OTHER_ALLOWED");
    }

    #[test]
    fn test_config_format_detection() {
        assert_eq!(
            ConfigFormat::from_path(std::path::Path::new("config.json")),
            Some(ConfigFormat::Json)
        );
        assert_eq!(
            ConfigFormat::from_path(std::path::Path::new("config.toml")),
            Some(ConfigFormat::Toml)
        );
        assert_eq!(
            ConfigFormat::from_path(std::path::Path::new("config.yaml")),
            Some(ConfigFormat::Yaml)
        );
    }

    #[test]
    fn test_file_config_error_preserves_parse_source() {
        let err: FileConfigError = serde_yaml::from_str::<Value>("[invalid").unwrap_err().into();
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_merge_configs_merges_object_sources() {
        let first = StaticLoader { name: "first", value: serde_json::json!({ "a": 1 }) };
        let second = StaticLoader { name: "second", value: serde_json::json!({ "b": 2 }) };

        let merged = merge_configs::<Value>(&[&first, &second]).unwrap();
        assert_eq!(merged, serde_json::json!({ "a": 1, "b": 2 }));
    }

    #[test]
    fn test_merge_configs_deep_merges_nested_objects() {
        let first = StaticLoader {
            name: "first",
            value: serde_json::json!({ "database": { "host": "localhost", "port": 5432 } }),
        };
        let second = StaticLoader {
            name: "second",
            value: serde_json::json!({ "database": { "port": 6432 } }),
        };

        let merged = merge_configs::<Value>(&[&first, &second]).unwrap();

        assert_eq!(
            merged,
            serde_json::json!({ "database": { "host": "localhost", "port": 6432 } })
        );
    }

    #[test]
    fn test_merge_configs_rejects_non_object_sources() {
        let loader = StaticLoader { name: "array-source", value: serde_json::json!(["invalid"]) };

        let err = merge_configs::<Value>(&[&loader]).unwrap_err();
        assert!(err.to_string().contains("array-source"));
        assert!(err.to_string().contains("array"));
    }

    #[test]
    fn test_merge_configs_propagates_loader_errors() {
        let err = merge_configs::<Value>(&[&ErrorLoader]).unwrap_err();
        assert!(err.to_string().contains("invalid config"));
    }
}
