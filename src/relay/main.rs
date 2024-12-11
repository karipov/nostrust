use core::message::ClientMessage;
use db::DataHolder;
use std::io::Cursor;
use tiny_http::{Request, Response, Server};

use sgx_isa::{Report, Targetinfo};

mod db;

fn post_echo(req: &mut Request, db: &mut DataHolder) -> Response<Cursor<Vec<u8>>> {
    let mut request_body_bytes = Vec::new();
    req.as_reader()
        .read_to_end(&mut request_body_bytes)
        .unwrap();

    // deserialize the event
    let message: ClientMessage = serde_json::from_slice(&request_body_bytes).unwrap();
    println!("Received: {:#?}", message);

    // handle the message
    let response = db.handle_message(message);
    match response {
        Some(events) => {
            let response_body = serde_json::to_vec(&events).unwrap();
            Response::from_data(response_body).with_status_code(200)
        }
        None => Response::from_string("OK").with_status_code(200),
    }
}

fn main() {
    let (ip, port) = ("0.0.0.0", 8080);
    let mut db = DataHolder::default();

    let targetinfo = Targetinfo::from(Report::for_self());
    println!("Attestation Measurement: {:?}", targetinfo.measurement);

    let server = Server::http(format!("{}:{}", ip, port)).unwrap();
    for mut request in server.incoming_requests() {
        let resp = post_echo(&mut request, &mut db);
        request.respond(resp).unwrap();
    }
}

// Next:
// Receive and parse
// Verify message integrity
// Store / retrieve / delete events on a db
// Send messages to clients
