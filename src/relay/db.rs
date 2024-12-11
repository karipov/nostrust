use core::{event::Event, message::ClientMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct DataHolder {
    pub events: HashMap<String, Vec<Event>>, // maps user -> list of their posts
    pub subscribers: HashMap<String, Vec<String>>, // maps user -> list of their subscribers
    pub subscriptions: HashMap<String, Vec<String>>, // maps user -> list of their subscriptions
}

impl DataHolder {
    // FIXME: Add some error handling
    fn add_event(&mut self, event: Event) {
        let user = event.pubkey.clone();

        self.events.entry(user.clone()).or_default().push(event);
    }

    fn add_subscription(&mut self, subscriber: String, author: String) {
        self.subscriptions
            .entry(subscriber.clone())
            .or_default()
            .push(author.clone());
        self.subscribers
            .entry(author.clone())
            .or_default()
            .push(subscriber.clone());

        // println!("Subscriptions: {:#?}", self.subscriptions);
        // println!("Subscribers: {:#?}", self.subscribers);
    }

    fn delete_subscription(&mut self, user: String, subscriber: String) {
        if let Some(subscriptions) = self.subscriptions.get_mut(&user) {
            subscriptions.retain(|s| s != &subscriber);
        }
        if let Some(subscribers) = self.subscribers.get_mut(&subscriber) {
            subscribers.retain(|s| s != &user);
        }

        // println!("Subscriptions: {:#?}", self.subscriptions);
    }

    // GDPR deletion
    fn delete_events(&mut self, user: String) {
        self.events.remove(&user);
    }

    // fn get_events(&self, user: String) -> Option<&Vec<Event>> {
    //     // self.events.get(&user)
    // }

    pub fn handle_message(&mut self, message: ClientMessage) -> Option<Vec<Event>> {
        match message {
            // event can be a post, deletion
            ClientMessage::Event(event) => {
                if !event.verify() {
                    println!("Event failed verification");
                    return None;
                }

                match event.kind {
                    0 => print!("metadata"), // FIXME: deal w this
                    5 => {
                        // full deletion for the requesting user
                        self.delete_events(event.pubkey.clone());
                    }
                    _ => self.add_event(event),
                }
                None
            }
            ClientMessage::Req(user, filters) => {
                let filter = filters.first().unwrap().clone();
                let author = filter.authors.unwrap().first().unwrap().clone();
                let subscriber = user.clone();

                self.add_subscription(subscriber, author);
                None
            }
            ClientMessage::Close(user, filters) => {
                let filter = filters.first().unwrap().clone();
                let author = filter.authors.unwrap().first().unwrap().clone();
                let unsubscriber = user.clone();

                self.delete_subscription(unsubscriber, author);
                None
            }
            ClientMessage::Get(user) => {
                let subscriptions = self.subscriptions.get(&user).unwrap();
                let mut retreived_events: Vec<Event> = vec![];
                for subscription in subscriptions {
                    if let Some(events) = self.events.get(subscription) {
                        retreived_events.extend(events.clone());
                    }
                }
                // println!("All events: {:#?}", retreived_events);

                // send all events to the user
                Some(retreived_events)
            }
            ClientMessage::Info => {
                println!("Unsupported message type");
                None
            }
        }
    }
}
