// Sending events over HTTP

use httparse::{Request, EMPTY_HEADER};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::stdout;

mod event;
use event::Event;

fn convert_event(event: Event) {
    // Serialize the Event to JSON
    let json_string = match to_string(event) {
        Ok(json) => json,
        Err(_) => {
            eprintln!("Failed to serialize Event to JSON.");
            return String::new();
        }
    };

    // Build the HTTP request
    let mut request = String::new();
    writeln!(
        request,
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        path,
        host,
        json_string.len(),
        json_string
    )
    .unwrap();

    request
}

