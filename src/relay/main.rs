use core::event::Event;
use serde::{Deserialize, Serialize};
use core::message::RelayMesage;

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub enum RelayMesage {
//     Event(SubscriptionId, Event),
//     Ok(EventId, bool, String),
//     Eose(SubscriptionId),
//     Closed(SubscriptionId, String),
//     Notice(String),
// }

fn main() {
    println!("Hello, world!");
}

// test suite
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_message() {
        let event = Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            created_at: 0,
            kind: 0,
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        };
        let original = RelayMesage::Event("sub_id".to_string(), event.clone());
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: RelayMesage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, original);
    }
}
