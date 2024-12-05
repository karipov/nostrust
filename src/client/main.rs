use anyhow::Result;

use crate::terminal::{Command::*, SimplerTheme, TerminalInput};
use crate::keys::generate_keypair;
use dialoguer::{console::Style, Input};

// mod message;
// use core::message::ClientMessage;
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

    loop {
        let input: TerminalInput = Input::with_theme(&SimplerTheme::default())
            .with_prompt("> ")
            .interact_text()
            .unwrap();

        // TODO: create registration flow for private and public keys
        match input.command {
            Post => println!("post"), // Steps here: create event using Event.new, sign it, send it to the relay, await and verify success
            Follow => println!("follow"), // Steps here: create filter using Filter.one_author, send request to relay, await and print relay response
            Unfollow => println!("unfollow"), // Steps here: send close request to relay, await and print relay response
            Delete => println!("delete"), // Steps here: send delete event (kind 5) to relay, await and verify success
            Get => println!("get"), // Steps here: send request to relay, await events and print them
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