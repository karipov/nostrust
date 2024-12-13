use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub banner: Option<String>,
    pub icon: Option<String>,
    pub contact: Option<String>,
    pub supported_nips: Vec<usize>,
    pub software: String,
    pub version: String,
    pub attestation: [u8; 32],
}