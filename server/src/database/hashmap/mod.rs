use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashMapRecord<T> {
    id: Uuid,
    data: T,
}

// impl<T> RecordLike<T> for HashMapRecord<T> 
// where 
//     T: Serialize + DeserializeOwned + Send + Sync + 'static + std::fmt::Debug
// {
//     fn id(&self) -> Uuid {
//         self.id
//     }

//     fn into_inner(self) -> T {
//         self.data
//     }
// }

#[derive(Debug, Clone)]
pub struct HashMapDb {
    data: Arc<RwLock<HashMap<Uuid, Vec<u8>>>>,
}

impl Default for HashMapDb {
    fn default() -> Self {
        Self::new()
    }
}

impl HashMapDb {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// #[async_trait]
// impl Database for HashMapDb {
//     type FilterType = ();  // Simple implementation without filtering
//     type PatchType = serde_json::Value;  // Using JSON for patches
//     type Error = CrayonServerError;
//     type Record<T> = HashMapRecord<T> where T: Serialize + DeserializeOwned + Send + Sync + 'static;

//     async fn get<T>(&self, id: Uuid) -> Result<Option<Self::Record<T>>, Self::Error>
//     where 
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let data = self.data.read().await;
//         if let Some(bytes) = data.get(&id) {
//             let record: T = serde_json::from_slice(bytes)?;
//             Ok(Some(HashMapRecord { id, data: record }))
//         } else {
//             Ok(None)
//         }
//     }

//     async fn create<T>(&self, record: T) -> Result<Uuid, Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         let id = Uuid::new_v4();
//         let bytes = serde_json::to_vec(&record)?;
//         data.insert(id, bytes);
//         Ok(id)
//     }

//     async fn delete<T>(&self, id: Uuid) -> Result<(), Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         data.remove(&id);
//         Ok(())
//     }

//     async fn list<T>(&self) -> Result<Vec<Self::Record<T>>, Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let data = self.data.read().await;
//         let mut records = Vec::new();
//         for (id, bytes) in data.iter() {
//             let record: T = serde_json::from_slice(bytes)?;
//             records.push(HashMapRecord {
//                 id: *id,
//                 data: record,
//             });
//         }
//         Ok(records)
//     }

//     async fn update<T>(&self, id: Uuid, record: T) -> Result<Self::Record<T>, Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         let bytes = serde_json::to_vec(&record)?;
//         data.insert(id, bytes);
//         Ok(HashMapRecord { id, data: record })
//     }

//     async fn patch<T>(&self, id: Uuid, partial: Self::PatchType) -> Result<Self::Record<T>, Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         if let Some(bytes) = data.get(&id) {
//             let mut current: serde_json::Value = serde_json::from_slice(bytes)?;
//             json_patch::merge(&mut current, &partial);
//             let updated: T = serde_json::from_value(current)?;
//             let new_bytes = serde_json::to_vec(&updated)?;
//             data.insert(id, new_bytes);
//             Ok(HashMapRecord { id, data: updated })
//         } else {
//             Err(CrayonServerError::NotFound)
//         }
//     }

//     async fn bulk_create<T>(&self, records: Vec<T>) -> Result<Vec<Self::Record<T>>, Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         let mut results = Vec::new();
//         for record in records {
//             let id = Uuid::new_v4();
//             let bytes = serde_json::to_vec(&record)?;
//             data.insert(id, bytes);
//             results.push(HashMapRecord { id, data: record });
//         }
//         Ok(results)
//     }

//     async fn bulk_delete<T>(&self, ids: Vec<Uuid>) -> Result<(), Self::Error>
//     where
//         T: Serialize + DeserializeOwned + Send + Sync + 'static 
//     {
//         let mut data = self.data.write().await;
//         for id in ids {
//             data.remove(&id);
//         }
//         Ok(())
//     }
// }