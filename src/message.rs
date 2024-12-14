// use core::event::Event;
// use core::filter::Filter;
use crate::event::Event;
use crate::filter::Filter;
use crate::info::Info;
use serde::{Deserialize, Serialize};

// #[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String, Vec<Filter>),
    Info,
    Get(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelayMessage {
    Events(Vec<Event>),
    Info(Info),
}

// testing to see what the messages look like
#[cfg(test)]
mod tests {
    use super::*;

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
        let original = RelayMessage::Events(vec![event.clone()]);
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: RelayMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, original);
    }
}
