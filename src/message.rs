// use core::event::Event;
// use core::filter::Filter;
use crate::event::Event;
use crate::filter::Filter;
use serde::{Deserialize, Serialize};

// #[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Event(Event),
    Req(String, Vec<Filter>),
    Close(String),
}

type SubscriptionId = String;
type EventId = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelayMesage {
    Event(SubscriptionId, Event),
    Ok(EventId, bool, String),
    Eose(SubscriptionId),
    Closed(SubscriptionId, String),
    Notice(String),
}