use anyhow::Result;

use crate::keys::{generate_users, generate_subscription_id};
use crate::terminal::{Command::*, SimplerTheme, TerminalInput};
use core::event::Event;
use core::filter::Filter;
use core::message::{send_http_message, ClientMessage};
use dialoguer::{console::Style, Input};

// mod message;
mod keys;
mod terminal;

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
    let mut to_sub_id = std::collections::HashMap::<String, String>::new();

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
                let pubkey_str = hex::encode(author_pubkey.serialize());
                let filter = Filter::one_author(pubkey_str.clone());
                let subscription_id = generate_subscription_id();

                to_sub_id.insert(pubkey_str.clone(), subscription_id.clone());

                let message = ClientMessage::Req(subscription_id, vec![filter]);

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
                let pubkey_str = hex::encode(author_pubkey.serialize());
                if let Some(subscription_id) = to_sub_id.get(&pubkey_str) {
                    let message = ClientMessage::Close(subscription_id.clone());
                    to_sub_id.remove(&pubkey_str);
                    send_http_message(ip, port, message);
                } else {
                    println!("You are not subscribed to this user!"); // FIXME: this should be an error message
                }

            } // Steps here: send close request to relay, await and print relay response
            Delete => println!("delete"), // Steps here: send delete event (kind 5) to relay, await and verify success
            Get => {
                let filter = Filter::default();
                let subscription_id = "sub_id".to_string(); // FIXME: what is subscription_id here lol

                let message = ClientMessage::Req(subscription_id, vec![filter]);

                send_http_message(ip, port, message);
            } // Steps here: send request to relay, await events and print them
            Info => println!("info"),     // Steps here: print info about the relay
            Help => println!(
                "The following commands are available: {}",
                [Post, Follow, Unfollow, Help, Quit, Delete, Get, Info]
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
