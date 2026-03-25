//! # Query Ports
//!
//! Query ports define read operations (CQRS pattern).

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// A query represents a request for data (read-only).
pub trait Query: Send + Sync + Serialize + 'static {
    /// The type of the result.
    type Result: for<'de> Deserialize<'de>;
}

/// Query handler port.
#[async_trait::async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Handle a query and return a result.
    async fn handle(&self, query: Q) -> Result<Q::Result>;
}

/// Query bus for dispatching queries to handlers.
#[async_trait::async_trait]
pub trait QueryBus: Send + Sync {
    /// The query type.
    type Query: Query;

    /// Dispatch a query to its handler.
    async fn execute(&self, query: Self::Query) -> Result<<Self::Query as Query>::Result>;
}

/// Paginated result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, page: u32, page_size: u32, total: u64) -> Self {
        Self { items, page, page_size, total }
    }

    pub fn total_pages(&self) -> u32 {
        ((self.total + self.page_size as u64 - 1) / self.page_size as u64) as u32
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}
