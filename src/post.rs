use jiff::civil::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub labels: Vec<String>,
    pub date: Date,
    #[serde(default)]
    pub stage: PostStage,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostStage {
    #[default]
    Published,
    Draft,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub metadata: Metadata,
    pub slug: String,
    pub content: String,
}
