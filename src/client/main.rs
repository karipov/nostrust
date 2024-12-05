use core::event::Event;
use core::filter::Filter;

pub enum ClientMessage {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String),
}

fn main() {
    // setup private key
    // connect to the relay
    // send events
    // receive events
}
