//! # HTTP Ports
//!
//! HTTP ports define HTTP client operations.

use crate::error::Result;
use serde::Serialize;

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// HTTP request.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<std::time::Duration>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: std::collections::HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_json_body<T: Serialize>(mut self, body: &T) -> Result<Self> {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = Some(serde_json::to_vec(body)?);
        Ok(self)
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// HTTP response.
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}

/// HTTP client port.
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync {
    /// Send an HTTP request.
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse>;

    /// Send a GET request.
    async fn get(&self, url: &str) -> Result<HttpResponse> {
        let request = HttpRequest::new(HttpMethod::Get, url);
        self.send(request).await
    }

    /// Send a POST request with JSON body.
    async fn post_json<T: Serialize + std::marker::Sync>(&self, url: &str, body: &T) -> Result<HttpResponse> {
        let request = HttpRequest::new(HttpMethod::Post, url).with_json_body(body)?;
        self.send(request).await
    }
}
