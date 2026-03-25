//! # Phenotype HTTP Adapter
//!
//! HTTP client adapter implementing the `HttpClient` port for hexagonal architecture.
//!
//! ## Features
//!
//! - **Async/Await**: Fully async implementation using `reqwest`
//! - **TLS Support**: Secure connections with `rustls`
//! - **JSON Support**: Built-in JSON serialization/deserialization
//! - **Timeout Support**: Configurable request timeouts

pub mod error;
pub mod http_client;
pub mod http_config;

pub use error::{HttpError, Result};
pub use http_client::ReqwestHttpClient;
pub use http_config::HttpConfig;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        assert!(true);
    }
}
