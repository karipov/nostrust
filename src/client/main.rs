use anyhow::Result;

use crate::terminal::{Command::*, SimplerTheme, TerminalInput};
use crate::keys::generate_users;
use dialoguer::{console::Style, Input};
use core::event::Event;
use core::message::{self, ClientMessage};
use core::filter::{self, Filter};

// mod message;
mod terminal;
mod keys;

// THIS IS A SINGLE-RELAY CLIENT
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
    let user_id = "@komron"; // TODO: get user_id from user input

    // println!("Users:");
    // for (user_id, credentials) in users.iter() {
    //     println!("{}: {}, {}", user_id, hex::encode(credentials.public_key.serialize()), hex::encode(credentials.private_key.secret_bytes()));
    // }

    loop {
        let input: TerminalInput = Input::with_theme(&SimplerTheme::default())
            .with_prompt("> ")
            .interact_text()
            .unwrap();

        // TODO: create registration flow for private and public keys
        match input.command {
            Post => 
            {
                // println!("{:?}", input.argument.unwrap());
                let content = input.argument.unwrap();
                let credentials = users.get(user_id).unwrap();
                // println!("User: {}, Public Key: {}, Private Key: {}, Content: {}", user_id, hex::encode(credentials.public_key.serialize()), hex::encode(credentials.private_key.secret_bytes()), content);

                let event = Event::new(
                    hex::encode(credentials.private_key.secret_bytes()),
                    hex::encode(credentials.public_key.serialize()),
                    1, // What about 0?
                    vec![],
                    content
                );

                // println!("{:?}", event);

                let message = ClientMessage::Event(event);

                // TODO: send message to relay, await and print relay response
            },
            Follow => {
                let author = input.argument.unwrap();
                let author_pubkey = users.get(&author).unwrap().public_key;
                let filter = Filter::one_author(hex::encode(author_pubkey.serialize()));
                let subscription_id = "sub_id".to_string(); // FIXME: generate subscription_id

                println!("{:?}", filter);

                let message = ClientMessage::Req(subscription_id, vec![filter]);

            }, // Steps here: create filter using Filter.one_author, send request to relay, await and print relay response
            Unfollow => {
                // Can only do this if we have a subscription_id
            }, // Steps here: send close request to relay, await and print relay response
            Delete => println!("delete"), // Steps here: send delete event (kind 5) to relay, await and verify success
            Get => {
                let filter = Filter::default();
                let subscription_id = "sub_id".to_string(); // FIXME: generate subscription_id or something

                let message = ClientMessage::Req(subscription_id, vec![filter]);
            }, // Steps here: send request to relay, await events and print them
            Info => println!("info"), // Steps here: print info about the relay
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