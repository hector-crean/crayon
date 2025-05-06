



#[derive(Debug)]
pub struct TextSearchIndexConfig<'a> {
    pub analyzer_name: &'a str,
    pub tokenizers: Vec<&'a str>,
    pub filters: Vec<&'a str>,
    pub algorithm: &'a str,
    pub enable_highlights: bool,
}

impl Default for TextSearchIndexConfig<'_> {
    fn default() -> Self {
        Self {
            analyzer_name: "text_analyzer",
            tokenizers: vec!["blank"],
            filters: vec!["lowercase", "snowball(english)"],
            algorithm: "BM25",
            enable_highlights: true,
        }
    }
}

impl TextSearchIndexConfig<'_> {
    pub fn build_analyzer_query(&self) -> String {
        format!(
            "DEFINE ANALYZER {} TOKENIZERS {} FILTERS {}",
            self.analyzer_name,
            self.tokenizers.join(", "),
            self.filters.join(", ")
        )
    }

    pub fn build_index_query(&self, table_id: &str, field: &str) -> String {
        format!(
            "DEFINE INDEX idx_{}_{} 
            ON {} 
            FIELDS {}
            SEARCH ANALYZER {} {} {}",
            table_id,
            field,
            table_id,
            field,
            self.analyzer_name,
            self.algorithm,
            if self.enable_highlights {
                "HIGHLIGHTS"
            } else {
                ""
            }
        )
    }
}
