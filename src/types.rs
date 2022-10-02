use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page_no: u32,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub title: String,
    pub contents: Vec<String>,
}
