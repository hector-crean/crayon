

#[derive(Debug)]
pub struct SurrealFilter {
    // Add fields as needed for filtering
    pub query: String,
    pub params: Option<serde_json::Value>,
}