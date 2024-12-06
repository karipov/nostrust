use core::message::ClientMessage;
use std::{collections::HashMap, io::Cursor};
use tiny_http::{Request, Response, Server};

type RouteHandler = fn(&mut Request) -> Response<Cursor<Vec<u8>>>;

fn get_hello(_req: &mut Request) -> Response<Cursor<Vec<u8>>> {
    Response::from_string("Hello from /hello!")
}

fn post_echo(req: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let mut request_body_bytes = Vec::new();
    req.as_reader()
        .read_to_end(&mut request_body_bytes)
        .unwrap();
    // let request_body_string = String::from_utf8(request_body_bytes.clone()).unwrap();

    // deserialize the event
    let message: ClientMessage = serde_json::from_slice(&request_body_bytes).unwrap();
    println!("Received: {:#?}", message);

    if let ClientMessage::Event(event) = message {
        println!("Is event verified?: {:#?}", event.verify());
    }

    // send success back
    Response::from_string("OK").with_status_code(200)
}

fn main() {
    let (ip, port) = ("0.0.0.0", 8080);
    let routes = HashMap::from([
        ("POST:/".to_string(), post_echo as RouteHandler),
        ("POST:/hello".to_string(), get_hello as RouteHandler),
    ]);

    let server = Server::http(format!("{}:{}", ip, port)).unwrap();
    for mut request in server.incoming_requests() {
        let route_key = format!("{}:{}", request.method(), request.url());
        match routes.get(&route_key) {
            Some(handler) => {
                let response = handler(&mut request);
                request.respond(response).unwrap();
            }
            None => {
                let response = Response::from_string("404 Not Found\n").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}

// test suite
#[cfg(test)]
mod tests {
    use super::*;
    use core::event::Event;
    use core::message::RelayMessage;

    #[test]
    fn test_relay_message() {
        let event = Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            created_at: 0,
            kind: 0,
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        };
        let original = RelayMessage::Event("sub_id".to_string(), event.clone());
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: RelayMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, original);
    }
}

// Next:
// Receive and parse
// Verify message integrity
// Store / retrieve / delete events on a db
// Send messages to clients
