use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub ids: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub kinds: Option<Vec<u32>>,
    #[serde(flatten)] // Point of this?
    pub tags: Option<std::collections::HashMap<String, Vec<String>>>, // Need to double check this
    pub since: Option<u64>,
    pub until: Option<u64>,
    pub limit: Option<u32>,
}