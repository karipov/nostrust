use anyhow::Result;
use keys::get_user_by_pubkey;

use crate::keys::generate_users;
use crate::terminal::{Command::*, SimplerTheme, TerminalInput};
use chrono::{Local, TimeZone};
use core::event::Event;
use core::filter::Filter;
use core::info::Info;
use core::message::ClientMessage;
use dialoguer::{console::Style, Input};
use serde::Serialize;
use sha2::Digest;

// mod message;
mod keys;
mod terminal;

pub fn send_http_message(ip: &str, port: u16, message: impl Serialize) -> Option<Vec<u8>> {
    let output = reqwest::blocking::Client::new()
        .post(format!("http://{}:{}/", ip, port))
        .body(serde_json::to_string(&message).unwrap())
        .send();

    if let Ok(output) = output {
        if let Ok(output) = output.bytes() {
            Some(output.to_vec())
        } else {
            None
        }
    } else {
        None
    }
}

fn main() -> Result<()> {
    let dim = Style::new().for_stderr().dim();
    let motd = r#"
     _   _  ____   _____ _______ _____  _    _  _____ _______ 
    | \ | |/ __ \ / ____|__   __|  __ \| |  | |/ ____|__   __|
    |  \| | |  | | (___    | |  | |__) | |  | | (___    | |   
    | . ` | |  | |\___ \   | |  |  _  /| |  | |\___ \   | |   
    | |\  | |__| |____) |  | |  | | \ \| |__| |____) |  | |   
    |_| \_|\____/|_____/   |_|  |_|  \_\\____/|_____/   |_|   

    Welcome to the Nostrust client. Nostrust is a GDPR-compliant relay by design.
    Type `help` for a list of commands.
    "#;

    println!("{}", dim.apply_to(motd));

    let users = generate_users();
    let (ip, port) = ("localhost", 8080);

    let chosen_user: String = Input::with_theme(&SimplerTheme::default())
        .with_prompt("> log in: ")
        .validate_with(|input: &String| {
            if users.contains_key(input) {
                Ok(())
            } else {
                Err("user not found, try again.")
            }
        })
        .interact()
        .unwrap();

    let credentials = users.get(&chosen_user).unwrap();
    let privkey = hex::encode(credentials.private_key.secret_bytes());
    let pubkey = hex::encode(credentials.public_key.serialize());
    // let mut to_sub_id = std::collections::HashMap::<String, String>::new();
    // let mut sub_ids = std::collections::HashSet::<String>::new();

    loop {
        let input: TerminalInput = Input::with_theme(&SimplerTheme::default())
            .with_prompt("> ")
            .interact_text()
            .unwrap();

        // TODO: create registration flow for private and public keys
        match input.command {
            Post => {
                let content = input.argument.unwrap();

                let event = Event::new(
                    privkey.clone(),
                    pubkey.clone(),
                    1, // TODO: What about 0?
                    vec![],
                    content,
                );

                let message = ClientMessage::Event(event);

                send_http_message(ip, port, message);

                // TODO: send message to relay, await and print relay response
            }
            Follow => {
                let author = input.argument.unwrap();
                let author_pubkey = match users.get(&author) {
                    Some(user) => user.public_key,
                    None => {
                        println!("Author not found, try again."); // FIXME: this should be an error message
                        continue;
                    }
                };
                let author_pubkey_str = hex::encode(author_pubkey.serialize());
                let user_pubkey_str = hex::encode(credentials.public_key.serialize());
                let filter = Filter::one_author(author_pubkey_str.clone());
                // let subscription_id = generate_subscription_id();

                // to_sub_id.insert(pubkey_str.clone(), subscription_id.clone());
                // sub_ids.insert(subscription_id.clone());

                let message = ClientMessage::Req(user_pubkey_str, vec![filter]);

                send_http_message(ip, port, message);
            } // Steps here: create filter using Filter.one_author, send request to relay, await and print relay response
            Unfollow => {
                let author = input.argument.unwrap();
                let author_pubkey = match users.get(&author) {
                    Some(user) => user.public_key,
                    None => {
                        println!("Author not found, try again."); // FIXME: this should be an error message
                        continue;
                    }
                };
                let author_pubkey_str = hex::encode(author_pubkey.serialize());
                let user_pubkey_str = hex::encode(credentials.public_key.serialize());
                let filter = Filter::one_author(author_pubkey_str.clone());

                let message = ClientMessage::Close(user_pubkey_str, vec![filter]);

                send_http_message(ip, port, message);
            } // Steps here: send close request to relay, await and print relay response
            Delete => {
                let event = Event::new(
                    privkey.clone(),
                    pubkey.clone(),
                    5,
                    vec![],
                    "deletion request".to_string(),
                );
                let message = ClientMessage::Event(event);
                send_http_message(ip, port, message);
            } // Steps here: send delete event (kind 5) to relay, await and verify success
            Get => {
                let user_pubkey_str = hex::encode(credentials.public_key.serialize());
                let output_data =
                    send_http_message(ip, port, ClientMessage::Get(user_pubkey_str.clone()));

                if let Some(data) = output_data {
                    let events: Result<Vec<Event>, _> = serde_json::from_slice(&data);

                    #[allow(deprecated)]
                    if let Ok(events) = events {
                        for event in events {
                            println!(
                                "{} posted {:#?} at {}",
                                get_user_by_pubkey(event.pubkey.as_ref(), &users).unwrap(),
                                event.content,
                                Local
                                    .timestamp(event.created_at as i64, 0)
                                    .format("%d/%m/%y at %l:%M%P")
                            );
                        }
                    }
                }
            } // Steps here: send request to relay, await events and print them
            #[allow(deprecated)]
            Info => {
                let output_info =
                    send_http_message(ip, port, ClientMessage::Info);
                    let info: Result<Info, _> = serde_json::from_slice(&output_info.unwrap());
                    // hash and base-64 encode the info.attestation:
                    if let Ok(info) = info {
                        let measurement = base64::encode(
                            sha2::Sha256::digest(info.attestation)
                        );
                        println!("Relay Info:");
                        println!("Name: {}", info.name);
                        println!("Version: {}", info.version);
                        println!("Description: {}", info.description);
                        println!("Attestation: {}", measurement);
                        println!("Icon: {}", info.icon.unwrap());
                        println!("Software: {}", info.software);
                    }
            } // Steps here: print info about the relay
            Help => println!(
                "The following commands are available: {}",
                [Post, Follow, Unfollow, Get, Delete, Info, Help, Quit]
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Quit => break,
            // _ => (),
        }
    }

    Ok(())
}

// Now:
// Generate keys for client
// functions to handle each of post, follow, unfollow, delete, get
// send via HTTP to relay
