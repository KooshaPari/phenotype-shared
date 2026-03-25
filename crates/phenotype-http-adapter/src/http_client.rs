//! # HTTP Client
//!
//! Implementation of the `HttpClient` port using `reqwest`.

use async_trait::async_trait;

use phenotype_port_interfaces::outbound::http::{
    HttpBody, HttpClient, HttpRequest, HttpResponse, HttpStatus,
};
use phenotype_port_interfaces::error::{PortError, Result as PortResult};

use crate::error::{HttpError, Result};
use crate::http_config::HttpConfig;

/// Reqwest implementation of the HttpClient port.
#[derive(Clone)]
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    /// Create a new client with default config.
    pub fn new() -> Self {
        Self::from_config(&HttpConfig::default())
    }

    /// Create a client from config.
    pub fn from_config(config: &HttpConfig) -> Self {
        let mut builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs));

        if let Some(ref ua) = config.user_agent {
            builder = builder.user_agent(ua);
        }

        for (key, value) in &config.default_headers {
            builder = builder.default_header(key.as_str(), value.as_str());
        }

        Self {
            client: builder.build().expect("Failed to build HTTP client"),
        }
    }

    /// Create with a custom reqwest client.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    fn map_error(e: reqwest::Error) -> PortError {
        if e.is_timeout() {
            PortError::Timeout(e.to_string())
        } else if e.is_connect() {
            PortError::ConnectionError(e.to_string())
        } else {
            PortError::StorageError(e.to_string())
        }
    }
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    type Error = PortError;

    async fn request(&self, req: HttpRequest) -> Result<HttpResponse, Self::Error> {
        let mut builder = self.client.request(
            reqwest::Method::from_bytes(req.method.as_bytes()).unwrap_or(reqwest::Method::GET),
            &req.url,
        );

        for (key, value) in req.headers {
            builder = builder.header(key.as_str(), value.as_str());
        }

        if let Some(body) = req.body {
            builder = builder.body(body);
        }

        let response = builder
            .send()
            .await
            .map_err(Self::map_error)?;

        let status = response.status().as_u16();
        let headers: std::collections::HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(Self::map_error)?
            .to_vec();

        Ok(HttpResponse {
            status: HttpStatus(status),
            headers,
            body: Some(body),
        })
    }

    async fn get(&self, url: &str) -> Result<HttpResponse, Self::Error> {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: None,
        };
        self.request(req).await
    }

    async fn post(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, Self::Error> {
        let req = HttpRequest {
            method: "POST".to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: Some(body),
        };
        self.request(req).await
    }

    async fn put(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, Self::Error> {
        let req = HttpRequest {
            method: "PUT".to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: Some(body),
        };
        self.request(req).await
    }

    async fn delete(&self, url: &str) -> Result<HttpResponse, Self::Error> {
        let req = HttpRequest {
            method: "DELETE".to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: None,
        };
        self.request(req).await
    }

    async fn patch(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, Self::Error> {
        let req = HttpRequest {
            method: "PATCH".to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: Some(body),
        };
        self.request(req).await
    }
}
