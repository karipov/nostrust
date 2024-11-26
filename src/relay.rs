// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write};
// use std::thread;
// use serde::{Serialize, Deserialize};
use serde_json;

use crate::event::Event;
use crate::filter::{self, Filter};

// fn msg_to_event(msg: String) -> Event {
//     // Deserialize the Event from JSON
//     let parsed_msg: Vec<serde_json::Value> = serde_json::from_str(&msg).unwrap();
//     let event_json = parsed_msg[1].clone();
//     let deserialized: Event = serde_json::from_value(event_json).unwrap();

//     println!("Deserialized event: {:?}", deserialized);
//     return deserialized;
// } // likely not how this structure should be defined

fn str_to_filter(filter_str: String) -> Filter {
    // Deserialize the Filter from JSON
    let filter: Filter = serde_json::from_str(&filter_str).unwrap();
    println!("Deserialized filter: {:?}", filter);
    return filter;
}

fn parse_msg (msg: String) {
    // deserialize the message, if condition on index 0, call appropriate function
    let parsed_msg: Vec<serde_json::Value> = serde_json::from_str(&msg).unwrap();
    let msg_type = parsed_msg[0].as_str().unwrap();
    match msg_type {
        "EVENT" => {
            let event_json = parsed_msg[1].clone();
            let deserialized: Event = serde_json::from_value(event_json).unwrap();
            println!("Deserialized event: {:?}", deserialized);
            // Call whatever functions have to be called now
        }
        "FILTER" => {
            // Get subscription_id, then parse through all the filters somehow
        }
        "CLOSE" => {
            let subscription_id = parsed_msg[1].as_str().unwrap();
            // Call some kind of function to close the subscription
        }
        _ => {
            println!("Invalid message type");
        }
    }
}
