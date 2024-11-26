// Sending events over HTTP

// use httparse::{Request, EMPTY_HEADER};
// use serde_json::{json, Value};
// use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Sha256, Digest};
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::thread;
// use std::io::stdout;
// use base64;
use chrono;
use crate::event::Event;
use crate::filter::Filter;

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
    // return base64::encode(hash);
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

fn event_to_msg(event: Event) -> String {
    // Serialize the Event to JSON
    let msg = serde_json::json!(["EVENT", event]);
    let serialized = serde_json::to_string(&msg).unwrap();
    println!("serialized = {}", serialized);
    return serialized;
}

fn create_request() {

}

fn create_event_msg(pubkey: String, kind: u32, tags: Vec<Vec<String>>, content: String) -> String {
    let event = create_event(pubkey, kind, tags, content);
    let serialized = event_to_msg(event);

    return serialized;
}


fn create_req_msg() {

}

fn create_close_msg() {

}

// fn create_event_msg(pubkey: String, kind: u32, tags: Vec<Vec<String>>, content: String) -> String {
//     // Create an Event
//     let event = create_event(pubkey, kind, tags, content);

//     // Serialize the Event to JSON
//     let serialized = event_to_str(event);

    
// }

// fn create_msg(event_str: String, )