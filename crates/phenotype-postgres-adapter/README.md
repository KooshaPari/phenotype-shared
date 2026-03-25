# Phenotype Postgres Adapter

PostgreSQL adapter implementing the `Repository` port for hexagonal architecture.

## Features

- Connection pooling with `deadpool-postgres`
- Async/await with `tokio`
- Repository pattern implementation

## Usage

```rust
use phenotype_postgres_adapter::{PostgresRepository, PostgresConfig, create_pool};

#[tokio::main]
async fn main() {
    let config = PostgresConfig::default();
    let pool = create_pool(config).await.unwrap();
    let repo = PostgresRepository::with_default_table(pool);
    repo.initialize().await.unwrap();
}
```
