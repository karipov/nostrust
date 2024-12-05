use core::event::Event;
use core::filter::Filter;

#[allow(dead_code)]
pub enum Message {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String),
}
