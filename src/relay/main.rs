use core::message::{ClientMessage, RelayMessage};
use db::DataHolder;
use std::io::Cursor;
use tiny_http::{Request, Response, Server};

mod attestation;
mod db;
mod sealing;

/// The path to shutdown the server and SEAL the database.
/// This is just an example and should not be used in production.
/// Use CURL or similar to send a POST request to this path.
const ADMIN_PATH_SHUTDOWN: &str = "/super-secret-admin-path-shutdown";
/// The path to load a new database (UNSEAL).
const ADMIN_PATH_LOAD: &str = "/super-secret-admin-path-load";

/// Handle a request and return a response.
fn nostrust_response(req: &mut Request, db: &mut DataHolder) -> Response<Cursor<Vec<u8>>> {
    let path = req.url().to_string();
    if path == ADMIN_PATH_SHUTDOWN {
        println!("Shutting down and saving file...");
        db.to_filerunner();
        std::process::exit(0);
    } else if path == ADMIN_PATH_LOAD {
        println!("Loading new db...");
        *db = DataHolder::from_filerunner();
        return Response::from_string("OK").with_status_code(200);
    }

    // read request body
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
        Some(clientresponse) => {
            match clientresponse {
                RelayMessage::Events(events) => {
                    let response_body = serde_json::to_vec(&events).unwrap();
                    Response::from_data(response_body).with_status_code(200)
                }
                RelayMessage::Info(info) => {
                    let response_body = serde_json::to_vec(&info).unwrap();
                    Response::from_data(response_body).with_status_code(200)
                }
            }
        }
        None => Response::from_string("OK").with_status_code(200),
    }
}

fn main() {
    let (ip, port) = ("0.0.0.0", 8080);
    let mut db = DataHolder::default();

    let server = Server::http(format!("{}:{}", ip, port)).unwrap();
    for mut request in server.incoming_requests() {
        let resp = nostrust_response(&mut request, &mut db);
        request.respond(resp).unwrap();
    }
}
