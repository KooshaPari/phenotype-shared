//! # PostgreSQL Repository
//!
//! Implementation of the `Repository` port using PostgreSQL.

use async_trait::async_trait;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

use phenotype_port_interfaces::outbound::repository::{
    Delete, Entity, FindById, Repository, Save,
};
use phenotype_port_interfaces::error::{PortError, Result as PortResult};

use crate::error::{PostgresError, Result};
use crate::postgres_config::PostgresConfig;

/// PostgreSQL implementation of the Repository port.
#[derive(Clone)]
pub struct PostgresRepository {
    pool: Pool,
    table_name: String,
}

impl PostgresRepository {
    /// Create a new PostgresRepository with the given pool.
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            table_name: "entities".to_string(),
        }
    }

    /// Create with a custom table name.
    pub fn with_table(pool: Pool, table_name: impl Into<String>) -> Self {
        Self {
            pool,
            table_name: table_name.into(),
        }
    }

    /// Create a connection pool from config.
    pub async fn from_config(config: &PostgresConfig) -> Result<Self> {
        let pool = create_pool(config).await?;
        Ok(Self::new(pool))
    }

    /// Initialize the database schema.
    pub async fn initialize(&self) -> Result<()> {
        let client = self.pool.get().await.map_err(|e| {
            PostgresError::PoolError(e.to_string())
        })?;

        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                data JSONB NOT NULL,
                version INTEGER NOT NULL DEFAULT 1,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
            self.table_name
        );

        client.execute(&query, &[]).await.map_err(|e| {
            PostgresError::MigrationError(e.to_string())
        })?;

        // Create index
        let index_query = format!(
            "CREATE INDEX IF NOT EXISTS idx_{}_type ON {}(entity_type)",
            self.table_name, self.table_name
        );
        client.execute(&index_query, &[]).await.map_err(|e| {
            PostgresError::MigrationError(e.to_string())
        })?;

        Ok(())
    }

    fn map_error(e: deadpool_postgres::PoolError) -> PostgresError {
        PostgresError::PoolError(e.to_string())
    }
}

impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de>> FindById<T>
    for PostgresRepository
where
    T::Id: AsRef<str>,
{
}

/// Implementation of FindById for PostgreSQL
#[async_trait]
impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync> FindById<T>
    for PostgresRepository
where
    T::Id: AsRef<str> + std::fmt::Display,
{
    async fn find_by_id(&self, id: &T::Id) -> PortResult<Option<T>> {
        let client = self.pool.get().await.map_err(Self::map_error)?;

        let query = format!(
            "SELECT data FROM {} WHERE id = $1",
            self.table_name
        );

        let row = client
            .query_opt(&query, &[&id.as_ref()])
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get(0);
                let entity: T = serde_json::from_value(data).map_err(|e| {
                    PortError::InvalidData(e.to_string())
                })?;
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }
}

impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync> Save<T>
    for PostgresRepository
where
    T::Id: AsRef<str> + std::fmt::Display,
{
}

/// Implementation of Save for PostgreSQL
#[async_trait]
impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync> Save<T>
    for PostgresRepository
where
    T::Id: AsRef<str> + std::fmt::Display,
{
    async fn save(&self, entity: &T) -> PortResult<()> {
        let client = self.pool.get().await.map_err(Self::map_error)?;

        let data = serde_json::to_value(entity)
            .map_err(|e| PortError::InvalidData(e.to_string()))?;

        let query = format!(
            r#"
            INSERT INTO {} (id, entity_type, data, version)
            VALUES ($1, $2, $3, 1)
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                version = {}.version + 1,
                updated_at = NOW()
            "#,
            self.table_name, self.table_name
        );

        client
            .execute(
                &query,
                &[&T::entity_type().to_string(), &data],
            )
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(())
    }
}

impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync> Delete<T>
    for PostgresRepository
where
    T::Id: AsRef<str>,
{
}

/// Implementation of Delete for PostgreSQL
#[async_trait]
impl<T: Entity + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync> Delete<T>
    for PostgresRepository
where
    T::Id: AsRef<str>,
{
    async fn delete(&self, id: &T::Id) -> PortResult<()> {
        let client = self.pool.get().await.map_err(Self::map_error)?;

        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);

        client
            .execute(&query, &[&id.as_ref()])
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(())
    }
}

/// Create a connection pool from config.
pub async fn create_pool(config: &PostgresConfig) -> Result<Pool> {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.user = Some(config.user.clone());
    cfg.password = Some(config.password.clone());
    cfg.dbname = Some(config.database.clone());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| PostgresError::PoolError(e.to_string()))
}
