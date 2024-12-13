use core::{event::Event, message::{ClientMessage, RelayMessage}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use core::info::Info;
use sgx_isa::{Report, Targetinfo};

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::sealing::{self, SealData};

const FILERUNNER_SERVER: &str = "0.0.0.0:5555";

fn get_file(endpoint: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(FILERUNNER_SERVER)?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        endpoint
    );
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // remove the headers
    let response = response.split("\r\n\r\n").collect::<Vec<&str>>()[1].to_string();

    Ok(response)
}

fn set_file(endpoint: &str, body: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(FILERUNNER_SERVER)?;

    let request = format!(
        "POST {} HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        endpoint,
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
        let raw_db = get_file("/get-db").unwrap();
        let raw_seal_data = get_file("/get-sealdata").unwrap();
        let seal_data: SealData = serde_json::from_str(&raw_seal_data).unwrap();
        let seal_key = sealing::unseal_key(seal_data).unwrap();

        let decrypted_db = sealing::decrypt_string(&seal_key, raw_db).unwrap();

        println!("Retrieved db: {:#?}", decrypted_db);
        println!("Retrieved seal data: {:#?}", raw_seal_data);
        serde_json::from_str(&decrypted_db).unwrap()
    }

    // Serialize the db and send it to the filerunner
    pub fn to_filerunner(&self) {
        let raw_db = serde_json::to_string(&self).unwrap();
        let (seal_key, seal_data) = sealing::seal_key();

        let db = sealing::encrypt_string(&seal_key, raw_db).unwrap();
        let seal_data = serde_json::to_string(&seal_data).unwrap();

        set_file("/set-sealdata", &seal_data).unwrap();
        set_file("/set-db", &db).unwrap();
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

    pub fn handle_message(&mut self, message: ClientMessage) -> Option<RelayMessage> {
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
                Some(RelayMessage::Events(retreived_events))
            }
            ClientMessage::Info => {
                let targetinfo = Targetinfo::from(Report::for_self());
                let info = Info {
                    name: "Nostrust Relay".to_string(),
                    description: "An attestable GDPR-compliant Nostr relay!".to_string(),
                    icon: Some("https://drive.google.com/file/d/1AdM2UZaxVKjpm_6D45ktWc8wVg0ivxCV/view?usp=sharing".to_string()),
                    supported_nips: vec![1, 9, 11],
                    software: "https://github.com/karipov/nostrust".to_string(),
                    version: "0.1.0".to_string(),
                    attestation: targetinfo.measurement,
                    ..Default::default()
                };
                Some(RelayMessage::Info(info))
            }
        }
    }
}
