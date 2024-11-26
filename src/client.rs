// Sending events over HTTP

// use httparse::{Request, EMPTY_HEADER};
// use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
// use std::thread;
// use std::io::stdout;
use chrono;

// mod event;
// use event::Event;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64, // Unix timestamp
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

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

fn compute_id(pubkey: String, created_at: u64, kind: u32, tags: Vec<Vec<String>>, content: String) -> String {
    // Serialize the Event to JSON
    let serialized = serde_json::json!([
        0,
        pubkey,
        created_at,
        kind,
        tags,
        content
    ]);

    // Compute the SHA-256 hash of the serialized JSON
    let mut hasher = Sha256::new();
    hasher.update(serialized.to_string());
    let hash = hasher.finalize();

    // Return the hexadecimal representation of the hash
    return hex::encode(hash);
}

fn create_event(pubkey: String, kind: u32, tags: Vec<Vec<String>>, content: String) -> Event {
    // Create a new Event
    // Compute the ID of the Event
    let created_at = chrono::Utc::now().timestamp() as u64;
    let id = compute_id(pubkey.clone(), created_at.clone(), kind.clone(), tags.clone(), content.clone());
    let sig = id.clone();
    let event = Event {
        id,
        pubkey,
        created_at,
        kind,
        tags,
        content,
        sig
    };

    return event;
}

fn event_to_str(event: Event) -> String {
    // Serialize the Event to JSON
    let serialized = serde_json::to_string(&event).unwrap();
    println!("Serialized event: {}", serialized);
    return serialized;
}

// fn create_msg(event_str: String, )