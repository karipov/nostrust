// Sending events over HTTP
use crate::event::Event;
use crate::filter::Filter;

pub enum ClientMessage {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String),
}

fn something_here() {
    // setup private key
    // connect to the relay
    // send events
    // receive events
}
