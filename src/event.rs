// use std::net::TcpListener;
// use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl Event {
    pub fn compute_id(&self) -> String {
        let serialized = serde_json::json!([
            0,
            self.pubkey,
            self.created_at,
            self.kind,
            self.tags,
            self.content
        ]);
        let mut hasher = Sha256::new();
        hasher.update(serialized.to_string());
        
        return hex::encode(hasher.finalize());
    }

    pub fn verify(&self) -> bool {
        return self.id == self.compute_id();
    }
}