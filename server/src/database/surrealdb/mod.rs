pub mod error;
pub mod filter;
pub mod search;


use crate::database::UpsertCondition;

use surrealdb::sql::Thing;
use super::{Database, SearchOptions, SearchParams};
use axum::async_trait;
use error::SurrealError;
use filter::SurrealFilter;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    error::Db,
    Error, Surreal,
};
use uuid::Uuid;
use tracing;

#[derive(Clone)]
pub struct SurrealDatabase {
    client: Surreal<Client>,
}

#[derive(Debug)]
pub struct SurrealPatch {
    // Fields for partial updates
    pub updates: serde_json::Value,
}

pub struct SurrealConnectionOptions {
    url: String,
}

impl SurrealDatabase {
    pub fn new(client: Surreal<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Database for SurrealDatabase {
    type Error = SurrealError;
    type FilterType = SurrealFilter;
    type PatchType = SurrealPatch;
    type TableId = String;
    type RowId = String;
    type TextSearchIndexConfig<'search_index> = search::TextSearchIndexConfig<'search_index>;
    type Credentials<'credentials> = surrealdb::opt::auth::Root<'credentials>;
    type ConnectionOptions = SurrealConnectionOptions;

    async fn connect(options: Self::ConnectionOptions) -> Result<Self, Self::Error> {
        let client = Surreal::new::<Ws>(options.url.as_str()).await?;
        Ok(Self { client })
    }

    async fn authenticate<'credentials>(
        &self,
        credentials: Self::Credentials<'credentials>,
    ) -> Result<(), Self::Error> {
        self.client.signin(credentials).await?;
        Ok(())
    }

    async fn query<T>(
        &self,
        query: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
        
    {
        let mut result = match params {
            Some(p) => self.client.query(query).bind(p).await?,
            None => self.client.query(query).await?,
        };

        let values = result.take::<Vec<T>>(0)?;

        Ok(values)
    }


    // Add stubs for other required methods
    async fn subscribe<F, T>(&self, _query: &str, _callback: F) -> Result<(), Self::Error>
    where
        F: Fn(T) + Send + Sync + 'static,
        T: DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        todo!()
    }

    async fn upsert<T: Clone>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        record: T,
        condition: Option<UpsertCondition<T>>,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let (table_id, row_id) = record_id.clone();
        
        tracing::debug!(table = %table_id, id = %row_id, "Upserting record with specified ID");
        
        // Handle different upsert conditions
        match condition.unwrap_or_default() {
            UpsertCondition::ById => {
                // Use UPDATE with UPSERT to handle both insert and update cases
                let sql = "UPDATE $table:$id CONTENT $record UPSERT";
                
                let params = serde_json::json!({
                    "table": table_id,
                    "id": row_id.to_string(),
                    "record": record,
                });
                
                let mut result = self.client.query(sql).bind(params).await?;
                let updated: Option<T> = result.take(0)?;
                
                updated.ok_or_else(|| {
                    SurrealError::Surreal(Error::Db(Db::NoRecordFound))
                })
            },
            UpsertCondition::ByFields(fields) => {
                // For field-based matching, we need to construct a more complex query
                // that checks if a record with matching fields exists
                let field_conditions = fields.iter()
                    .map(|field| format!("r.{} = $record.{}", field, field))
                    .collect::<Vec<_>>()
                    .join(" AND ");
                
                let sql = format!(
                    "LET $existing = SELECT * FROM {} WHERE {} LIMIT 1;
                     RETURN IF $existing THEN 
                        (UPDATE $existing[0].id CONTENT $record)
                     ELSE 
                        (CREATE {} CONTENT $record)
                     END;",
                    table_id, field_conditions, table_id
                );
                
                let params = serde_json::json!({
                    "record": record,
                });
                
                let mut result = self.client.query(sql).bind(params).await?;
                let updated: Option<T> = result.take(0)?;
                
                updated.ok_or_else(|| {
                    SurrealError::Surreal(Error::Db(Db::NoRecordFound))
                })
            },
            UpsertCondition::Custom(_) => {
                // Custom comparison functions can't be serialized to the database
                // For SurrealDB, we'll need to fetch all records and do the comparison in Rust
                // This is inefficient but necessary for custom comparisons
                let records: Vec<T> = self.client.select(table_id.clone()).await?;
                
                // Since we can't extract the comparison function from UpsertCondition::Custom,
                // we'll fall back to ById behavior
                tracing::warn!("Custom comparison function not supported in SurrealDB implementation, falling back to ById");
                
                // Use UPDATE with UPSERT to handle both insert and update cases
                let sql = "UPDATE $table:$id CONTENT $record UPSERT";
                
                let params = serde_json::json!({
                    "table": table_id,
                    "id": row_id.to_string(),
                    "record": record,
                });
                
                let mut result = self.client.query(sql).bind(params).await?;
                let updated: Option<T> = result.take(0)?;
                
                updated.ok_or_else(|| {
                    SurrealError::Surreal(Error::Db(Db::NoRecordFound))
                })
            }
        }
    }

    async fn batch_insert<T>(
        &self,
        _table_id: Self::TableId,
        _records: Vec<T>,
    ) -> Result<Vec<(Self::TableId, Self::RowId)>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        todo!()
    }

    async fn wait_for_indexing(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn version(&self) -> Result<String, Self::Error> {
        Ok("1.0.0".to_string())
    }

    async fn get<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
    ) -> Result<Option<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let result: Option<T> = self.client.select(record_id).await?;
        Ok(result)
    }

    async fn get_many<T>(
        &self,
        table_id: Self::TableId,
        record_ids: Vec<Self::RowId>,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let things = record_ids.iter().map(|id| Thing::from((table_id.clone(), id.clone()))).collect::<Vec<_>>();
        let mut resp = self.client.query("SELECT * FROM type::table($table_id) WHERE id IN $thing_ids;").bind(("table_id", table_id)).bind(("thing_ids", things)).await?;
        let results: Vec<T> = resp.take(0)?;
        Ok(results)
    }

    async fn create<T>(
        &self,
        table_id: Self::TableId,
        record_id: Option<Self::RowId>,
        record: T,
    ) -> Result<(Self::TableId, Self::RowId), Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let record_id: String = record_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let created: Option<T> = self
            .client
            .create((table_id.clone(), record_id.to_string()))
            .content(record)
            .await?;

        if created.is_some() {
            Ok((table_id.clone(), record_id))
        } else {
            Err(SurrealError::Surreal(Error::Db(Db::NoRecordFound)))
        }
    }

    async fn delete<T>(&self, record_id: (Self::TableId, Self::RowId)) -> Result<(), Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let _: Option<T> = self.client.delete(record_id).await?;
        Ok(())
    }

    async fn list<T>(&self, table_id: Self::TableId) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let results: Vec<T> = self.client.select(table_id).await?;
        Ok(results)
    }

    async fn update<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        record: T,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let (_, row_id) = record_id.clone();

        let updated: Option<T> = self.client.update(record_id).content(record).await?;

        updated.ok_or_else(|| {
            SurrealError::Surreal(Error::Db(Db::IdNotFound {
                rid: row_id.to_string(),
            }))
        })
    }

    async fn patch<T>(
        &self,
        record_id: (Self::TableId, Self::RowId),
        partial: Self::PatchType,
    ) -> Result<T, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let (_, row_id) = record_id.clone();

        let updated: Option<T> = self.client.update(record_id).merge(partial.updates).await?;

        updated.ok_or_else(|| {
            SurrealError::Surreal(Error::Db(Db::IdNotFound {
                rid: row_id.to_string(),
            }))
        })
    }

    // Add this new struct to hold search options

    // async fn query<Q: IntoQuery>(query: Q, bindings:){}

    // Update the search method
    async fn search<T>(
        &self,
        table_id: Self::TableId,
        params: SearchParams,
        options: SearchOptions,
    ) -> Result<Vec<T>, Self::Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        tracing::debug!(?table_id, ?params, ?options, "Performing search");

        // If no fields are specified, search all fields with '*'
        let fields_condition = if options.fields.is_empty() {
            "*".to_string()
        } else {
            options.fields.join(" OR ")
        };

        tracing::trace!(fields_condition = ?fields_condition, "Constructed fields condition");

        let sql = format!(
            r#"
                SELECT *, search::score() as score 
                FROM {table_id} 
                WHERE ({fields_condition}) @@ $params.query
                {filters}
                {score_threshold}
                ORDER BY score DESC
                {limit}
                {offset}
                "#,
            table_id = table_id,
            fields_condition = fields_condition,
            filters = options
                .additional_filters
                .map(|f| format!(" AND {f}"))
                .unwrap_or_default(),
            score_threshold = options
                .score_threshold
                .map(|t| format!(" AND search::score() >= {t}"))
                .unwrap_or_default(),
            limit = options
                .limit
                .map(|l| format!(" LIMIT {l}"))
                .unwrap_or_default(),
            offset = options
                .offset
                .map(|o| format!(" START {o}"))
                .unwrap_or_default(),
        );

        tracing::debug!(sql = ?sql, "Executing search query");

        let mut result = self.client.query(sql).bind(("params", params)).await?;
        let results: Vec<T> = result.take(0)?;

        tracing::debug!(results_count = results.len(), "Search completed");
        Ok(results)
    }

    async fn text_search<'text, T>(
        &self,
        table_id: Self::TableId,
        fields: &[&str],
        config: Option<Self::TextSearchIndexConfig<'text>>,
    ) -> Result<Vec<T>, Self::Error>  where
    T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug,
    {
        let config = config.unwrap_or_default();

        // Define custom analyzer
        self.client.query(config.build_analyzer_query()).await?;

        let mut results: Vec<T> = vec![];


        // Create full-text search index for each field
        for field in fields {
            let define_index = config.build_index_query(&table_id, field);
            let mut resp = self.client.query(define_index).await?;
            let field_results: Vec<T> = resp.take(0)?;
            results.extend(field_results);
        }

        Ok(results)
    }
}
