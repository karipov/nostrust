// use core::event::Event;
// use core::filter::Filter;
use crate::event::Event;
use crate::filter::Filter;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

// #[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String, Vec<Filter>),
    Get(String), // pubkey, "all" or "subs"
    Info
}

type SubscriptionId = String;
type EventId = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelayMessage {
    Event(SubscriptionId, Event),
    Ok(EventId, bool, String),
    Eose(SubscriptionId),
    Closed(SubscriptionId, String),
    Notice(String),
}

pub fn send_http_message<T: Serialize>(ip: &str, port: u16, message: T) {
    let message_str = serde_json::to_string(&message).unwrap();
    let request = format!(
        "POST / HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        message_str.len(),
        message_str
    );
    println!("Request: {}", request);
    if let Ok(mut stream) = TcpStream::connect((ip, port)) {
        stream.write_all(request.as_bytes()).unwrap();
        println!("Sent message: {}", message_str);
    }
    else {
        println!("Failed to connect to the server");
    }
}

// testing to see what the request looks like
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_http_message() {
        let message = ClientMessage::Req("test".to_string(), vec![Filter::default()]);
        send_http_message("localhost", 8080, message);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_relay_message_serde() {
        let event = Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            created_at: 0,
            kind: 0,
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        };
        let original = RelayMessage::Event("sub_id".to_string(), event.clone());
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: RelayMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, original);
    }
}
