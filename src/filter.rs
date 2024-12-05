use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Filter {
    pub ids: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub kinds: Option<Vec<u32>>,
    pub tags: Option<HashMap<String, Vec<String>>>,
    pub since: Option<u64>,
    pub until: Option<u64>,
    pub limit: Option<u32>,
}

impl Filter {
    pub fn one_author(author: String) -> Self {
        Self {
            authors: Some(vec![author]),
            ..Default::default()
        }
    }
}
