use core::{event::Event, message::ClientMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::io::{Read, Write};
use std::net::TcpStream;

const FILERUNNER_SERVER: &str = "0.0.0.0:5555";

fn get_db() -> std::io::Result<String> {
    let mut stream = TcpStream::connect(FILERUNNER_SERVER)?;

    let request = "GET /get-db HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // remove the headers
    let response = response.split("\r\n\r\n").collect::<Vec<&str>>()[1].to_string();

    Ok(response)
}

fn set_db(body: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(FILERUNNER_SERVER)?;

    let request = format!(
        "POST /set-db HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    );

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    Ok(())
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DataHolder {
    pub events: HashMap<String, Vec<Event>>, // maps user -> list of their posts
    pub subscribers: HashMap<String, Vec<String>>, // maps user -> list of their subscribers
    pub subscriptions: HashMap<String, Vec<String>>, // maps user -> list of their subscriptions
}

impl DataHolder {
    // Retrieve the db from the filerunner and deserialize it
    pub fn from_filerunner() -> Self {
        let db = get_db().unwrap();
        println!("Retrieved db: {:#?}", db);
        serde_json::from_str(&db).unwrap()
    }

    // Serialize the db and send it to the filerunner
    pub fn to_filerunner(&self) {
        let db = serde_json::to_string(&self).unwrap();
        set_db(&db).unwrap();
    }

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
    }

    fn delete_subscription(&mut self, user: String, subscriber: String) {
        if let Some(subscriptions) = self.subscriptions.get_mut(&user) {
            subscriptions.retain(|s| s != &subscriber);
        }
        if let Some(subscribers) = self.subscribers.get_mut(&subscriber) {
            subscribers.retain(|s| s != &user);
        }
    }

    // GDPR deletion
    fn delete_events(&mut self, user: String) {
        self.events.remove(&user);
    }

    pub fn handle_message(&mut self, message: ClientMessage) -> Option<Vec<Event>> {
        match message {
            // event can be a post, deletion
            ClientMessage::Event(event) => {
                if !event.verify() {
                    println!("Event failed verification");
                    return None;
                }

                match event.kind {
                    // NIP-11
                    0 => print!("metadata"), // FIXME: deal w this
                    // NIP-09
                    5 => {
                        self.delete_events(event.pubkey.clone());
                    }
                    // NIP-01
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
