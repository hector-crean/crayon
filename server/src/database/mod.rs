pub mod hashmap;
pub mod surrealdb;

use std::fmt::Debug;

use axum::{async_trait, response::IntoResponse};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    needle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SearchOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub fields: Vec<String>,
    pub score_threshold: Option<f32>,
    pub additional_filters: Option<String>,
}

pub trait DatabaseError:
    std::error::Error + Send + Sync + 'static + IntoResponse + std::fmt::Debug
{
}
pub trait DatabaseTableId: Clone + Send + Sync + 'static + Into<String> + From<&'static str> + From<String> {}

impl DatabaseTableId for String {}

pub trait DatabaseRowId: Clone + Send + Sync + 'static + Into<String> +  From<&'static str> + From<String> {}

impl DatabaseRowId for String {}

/// Defines how to determine if a record exists for upsert operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpsertCondition<T> {
    /// Match by exact record ID
    ById,
    /// Match by specific fields
    ByFields(Vec<String>),
    /// Custom comparison function (serialized as null, only used in-memory)
    #[serde(skip)]
    Custom(fn(&T, &T) -> bool),
}

impl<T> Default for UpsertCondition<T> {
    fn default() -> Self {
        Self::ById
    }
}

/// A trait describing common database operations needed by the CMS.
/// This includes CRUD, bulk operations, searching, and additional optional operations.
#[async_trait]
pub trait Database: Send + Sync + 'static {
    /// An associated type representing the filter structure used to search records.
    type FilterType: Send + Sync;
    /// An associated type representing the structure used for partial updates (patches).
    type PatchType: Send + Sync;
    /// An associated type representing the error type returned by database operations.
    type Error: DatabaseError;
    type TableId: DatabaseTableId;
    type RowId: DatabaseRowId;
    /// Authentication credentials
    type Credentials<'credentials>: Send + Sync;
    type TextSearchIndexConfig<'text>: Send + Sync + Default;

    /// Connection options/configuration
    type ConnectionOptions: Send + Sync;

    /// Initialize and connect to the database
    async fn connect(options: Self::ConnectionOptions) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Authenticate with the database
    async fn authenticate<'credentials>(
        &self,
        credentials: Self::Credentials<'credentials>,
    ) -> Result<(), Self::Error>;

    /// Execute a raw query
    async fn query<T>(
        &self,
        query: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: DeserializeOwned + Send + Sync + 'static + Debug;

 
    /// Set up a live query subscription
    async fn subscribe<F, T>(&self, query: &str, callback: F) -> Result<(), Self::Error>
    where
        F: Fn(T) + Send + Sync + 'static,
        T: DeserializeOwned + Send + Sync + 'static + Debug;

    /// Upsert a record (create if not exists, update if exists)
    /// 
    /// The `condition` parameter determines how to check if a record already exists:
    /// - `UpsertCondition::ById` (default): Uses the provided record_id
    /// - `UpsertCondition::ByFields`: Checks equality on specified fields
    /// - `UpsertCondition::Custom`: Uses a custom comparison function
    async fn upsert<T: Clone>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        record: T,
        condition: Option<UpsertCondition<T>>,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Batch insert multiple records
    async fn batch_insert<T>(
        &self,
        table_id: Self::TableId,
        records: Vec<T>,
    ) -> Result<Vec<(Self::TableId, Self::RowId)>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Wait for all database operations to complete
    async fn wait_for_indexing(&self) -> Result<(), Self::Error>;

    /// Get database version
    async fn version(&self) -> Result<String, Self::Error>;

    /// Retrieve a single record by its UUID.
    async fn get<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
    ) -> Result<Option<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Get multiple records by their UUIDs.
    async fn get_many<T>(
        &self,
        table_id: Self::TableId,
        record_ids: Vec<Self::RowId>,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Create a new record in the database.
    /// 
    /// If `custom_id` is provided, it will be used as the record ID.
    /// Otherwise, a new ID will be generated.
    async fn create<T>(
        &self,
        table_id: Self::TableId,
        record_id: Option<Self::RowId>,
        record: T,
    ) -> Result<(Self::TableId, Self::RowId), Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Delete a record by its UUID.
    async fn delete<T>(&self, record_id: (Self::TableId, Self::RowId)) -> Result<(), Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// List all records.
    async fn list<T>(&self, table_id: Self::TableId) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Update an existing record by replacing it entirely.
    async fn update<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        record: T,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Partially update a record using a patch structure.
    async fn patch<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        partial: Self::PatchType,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Perform a full-text search on specified fields
    async fn search<T>(
        &self,
        table_id: Self::TableId,
        params: SearchParams,
        options: SearchOptions,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + Debug;

    /// Initialize full-text search capabilities for a table
    async fn text_search<'text, T>(
        &self,
        table_id: Self::TableId,
        fields: &[&str],
        config: Option<Self::TextSearchIndexConfig<'text>>,
    ) -> Result<Vec<T>, Self::Error>  where
    T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,;
}
