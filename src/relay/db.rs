use core::{event::Event, message::ClientMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct DataHolder {
    pub events: HashMap<String, Vec<Event>>, // maps user -> list of their posts
    pub subscribers: HashMap<String, Vec<String>>, // maps user -> list of their subscription_ids
    // pub subscriptions: HashMap<String, Vec<String>>, // maps subscription_id -> user being subscribed to
}

impl DataHolder {
    fn add_event(&mut self, event: Event) {
        let user = event.pubkey.clone();

        self.events.entry(user.clone()).or_default().push(event);

        // optional: Notify subscribers immediately
        if let Some(subscribers) = self.subscribers.get(&user) {
            for subscriber in subscribers {
                println!("Notify {} about new post from {}", subscriber, user);
            }
        }
    }

    fn add_subscription(&mut self, user: String, subscriber: String) {
        self.subscribers
            .entry(user.clone())
            .or_default()
            .push(subscriber.clone());

        // optional: send all existing posts to the new subscriber
        if let Some(events) = self.events.get(&user) {
            for event in events {
                println!("Send post from {} to new subscriber {}", user, subscriber);
            }
        }
    }

    // GDPR deletion
    fn delete_events(&mut self, user: String) {
        self.events.remove(&user);
    }

    pub fn handle_message(&mut self, message: ClientMessage) {
        match message {
            // event can be a post, deletion
            ClientMessage::Event(event) => {
                if !event.verify() {
                    println!("Event failed verification");
                    return;
                }

                match event.kind {
                    0 => print!("metadata"),
                    5 => {
                        // full deletion for the requesting user
                        self.delete_events(event.pubkey.clone());
                    }
                    _ => self.add_event(event),
                }
            }
            _ => println!("Unsupported message type"),
            // ClientMessage::Req(user: , _) => {
            //     self.add_subscription(user, subscriber);
            // }
        }
    }
}
