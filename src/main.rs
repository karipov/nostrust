use httparse::{Request, EMPTY_HEADER};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::stdout;
use chrono;

mod event;
use event::Event;

mod filter;
use filter::Filter;

mod client;
mod relay;

fn handle_client(mut stream: TcpStream) {
    println!("Handling client...");
    stdout().flush().unwrap(); // Ensure log is flushed

    let mut buffer = [0; 2048]; // Buffer for incoming request
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received {} bytes", size);
            stdout().flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return;
        }
    }

    // Parse the HTTP request
    let mut headers = [EMPTY_HEADER; 16];
    let mut request = Request::new(&mut headers);

    let result = request.parse(&buffer);
    print!("Parsing request...");
    match result {
        Ok(httparse::Status::Complete(body_start)) => {
            println!("Parsing complete, body starts at {}", body_start);
            stdout().flush().unwrap();

            let body = &buffer[body_start..];

            // Extract body based on Content-Length
            let content_length = request
                .headers
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case("Content-Length"))
                .and_then(|h| std::str::from_utf8(h.value).ok())
                .and_then(|v| v.parse::<usize>().ok());

            let body_text = if let Some(len) = content_length {
                &body[..len]
            } else {
                body
            };

            if let Ok(body_text) = std::str::from_utf8(body_text).map(str::trim) {
                println!("Raw Body: {}", body_text);
                stdout().flush().unwrap();

                // Parse JSON into serde_json::Value
                match serde_json::from_str::<Value>(body_text) {
                    Ok(parsed_json) => {
                        println!("Parsed JSON: {}", parsed_json);
                        stdout().flush().unwrap();

                        // Deserialize into Event struct
                        match serde_json::from_value::<Event>(parsed_json) {
                            Ok(event) => {
                                println!("Deserialized Event: {:?}", event);
                                stdout().flush().unwrap();
                            }
                            Err(err) => {
                                println!("Failed to deserialize into Event: {}", err);
                                stdout().flush().unwrap();
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to parse JSON: {}", err);
                        stdout().flush().unwrap();
                    }
                }
            } else {
                println!("Body is not valid UTF-8");
                stdout().flush().unwrap();
            }
        }
        Ok(httparse::Status::Partial) => {
            println!("Request parsing incomplete: Partial");
            stdout().flush().unwrap();
        }
        Err(err) => {
            println!("Failed to parse HTTP request: {}", err);
            stdout().flush().unwrap();
        }
    }

    // Send a response back
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nProcessed!";
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            println!("Response sent to client.");
            stdout().flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to send response: {}", e);
        }
    }
}

fn main() {
    println!("{:?}", chrono::offset::Utc::now());
    let port = 8080;

    // Start the server in a separate thread
    thread::spawn(move || {
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        println!("Server listening on port {}", port);
        stdout().flush().unwrap();

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                handle_client(stream);
            } else {
                eprintln!("Connection failed.");
            }
        }
    });

    // Give the server some time to start
    std::thread::sleep(std::time::Duration::from_millis(5_100));

    // Send a JSON request to the server
    let json_object = json!({
        "id": "5c62ef75a8e73ceb82aa83f363ddb151e91be5af57b7ee69913d74e4d0f68f89",
        "pubkey": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "created_at": 1732503636,
        "kind": 1,
        "tags": [
            ["e", "value1"],
            ["p", "value2"]
        ],
        "content": "This is a sample content for the event.",
        "sig": "0c7660fcf251a405afb3c6dbdf7217c0afac4dc945e1b246070b7e386d678ace"
    });

    let json_string = json_object.to_string();
    let request = format!(
        "POST / HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json_string.len(),
        json_string
    );

    print!("Connecting to server...");

    if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) {
        match stream.write_all(request.as_bytes()) {
            Ok(_) => {
                println!("Sent JSON to server:\n{}", request);
                stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
            }
        }
    } else {
        eprintln!("Failed to connect to server.");
    }
}


// // use httparse::Request;
// // use std::io::{Read, Write};
// // use std::net::TcpListener;

// mod event;
// use event::Event;
// use httparse::{Request, EMPTY_HEADER};
// use serde_json::{json, Value};
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::thread;


// /// Function to create and send a JSON response over TCP
// fn send_json(port: u16) {
//     let json_object = json!({
//         "pubkey": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
//         "created_at": 1732503636,
//         "kind": 1,
//         "tags": [
//             ["e", "value1"],
//             ["p", "value2"]
//         ],
//         "content": "This is a sample content for the event.",
//         "id": "5c62ef75a8e73ceb82aa83f363ddb151e91be5af57b7ee69913d74e4d0f68f89",
//         "sig": "0c7660fcf251a405afb3c6dbdf7217c0afac4dc945e1b246070b7e386d678ace"
//     });

//     let json_string = json_object.to_string();
//     let request = format!(
//         "POST / HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
//         json_string.len(),
//         json_string
//     );

//     // Connect to the server and send the HTTP request
//     if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) {
//         stream.write_all(request.as_bytes()).unwrap();
//         println!("Sent JSON to server:\n{}", request);
//     } else {
//         eprintln!("Failed to connect to server.");
//     }
// }

// /// Function to handle incoming TCP requests and parse JSON
// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 2048]; // Buffer for incoming request
//     stream.read(&mut buffer).unwrap();

//     // Parse the HTTP request
//     let mut headers = [EMPTY_HEADER; 16];
//     let mut request = Request::new(&mut headers);

//     let result = request.parse(&buffer);
//     if let Ok(_) = result {
//         // Get Content-Length header
//         print!("Getting content length...");
//         let content_length = request
//             .headers
//             .iter()
//             .find(|h| h.name.eq_ignore_ascii_case("Content-Length"))
//             .and_then(|h| std::str::from_utf8(h.value).ok())
//             .and_then(|v| v.parse::<usize>().ok());
//         print!("Parsing body...");
//             if let Ok(httparse::Status::Complete(body_start)) = result {
//                 let body = &buffer[body_start..];
            
//                 // Extract body based on Content-Length
//                 let body_text = if let Some(len) = content_length {
//                     &body[..len]
//                 } else {
//                     body
//                 };

//             // Convert body to string and parse JSON
//             if let Ok(body_text) = std::str::from_utf8(body_text).map(str::trim) {
//                 println!("Raw Body: {}", body_text);

//                 // Parse JSON
//                 print!("Parsing JSON...");
//                 match serde_json::from_str::<Value>(body_text) {
//                     Ok(parsed_json) => {
//                         println!("Parsed JSON: {}", parsed_json);
    
//                         // Deserialize into Event struct
//                         match serde_json::from_value::<Event>(parsed_json) {
//                             Ok(event) => {
//                                 println!("Deserialized Event: {:?}", event);
//                             }
//                             Err(err) => {
//                                 println!("Failed to deserialize into Event: {}", err);
//                             }
//                         }
//                     }
//                     Err(err) => {
//                         println!("Failed to parse JSON: {}", err);
//                     }
//                 }
//             } else {
//                 println!("Body is not valid UTF-8");
//             }
//         }

//         // Send a response back
//         let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nProcessed!";
//         stream.write_all(response.as_bytes()).unwrap();
//     } else {
//         println!("Failed to parse HTTP request");
//     }
// }

// /// Main function
// fn main() {
//     let port = 8080;

//     // Start the server in a separate thread
//     thread::spawn(move || {
//         let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
//         println!("Server listening on port {}", port);

//         for stream in listener.incoming() {
//             if let Ok(stream) = stream {
//                 handle_client(stream);
//             } else {
//                 eprintln!("Connection failed.");
//             }
//         }
//     });

//     // Give the server some time to start
//     std::thread::sleep(std::time::Duration::from_millis(100));

//     // Send JSON to the server
//     send_json(port);
// }

// fn send_json(port: u16) {
//     // create json object
//     let json_object = json!({
//         "pubkey": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
//         "created_at": 1732503636,
//         "kind": 1,
//         "tags": [
//             ["e", "value1"],
//             ["p", "value2"]
//         ],
//         "content": "This is a sample content for the event.",
//         "id": "5c62ef75a8e73ceb82aa83f363ddb151e91be5af57b7ee69913d74e4d0f68f89",
//         "sig": "0c7660fcf251a405afb3c6dbdf7217c0afac4dc945e1b246070b7e386d678ace"
//     });

//     // convert json object to string
//     let json_string = json_object.to_string();

//     // send to http port
//     let request = format!(
//         "POST / HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
//         json_string.len(),
//         json_string
//     );
    
//     if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) {
//         stream.write_all(request.as_bytes()).unwrap();
//         println!("Sent JSON to server:\n{}", request);
//     } else {
//         eprintln!("Failed to connect to server.");
//     }
// }
// fn main() {

//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         let mut buffer = [0; 1024]; // Allocate buffer for the incoming request
//         // UNDERSTAND IMPLICATIONS OF BUFFER SIZE

//         // Read the incoming request into the buffer
//         stream.read(&mut buffer).unwrap();

//         // Create structures to parse the request line and headers
//         let mut headers = [EMPTY_HEADER; 16];
//         let mut request = Request::new(&mut headers);

//         // Parse the HTTP request
//         let result = request.parse(&buffer);
//         if result.is_ok() {
//             println!("Headers: {:?}", request.headers);
            
//             // Extract the start of the body using the parsed headers
//             let body_start = result.unwrap().unwrap();
//             let body = &buffer[body_start..];

//             // Convert body to a string (if it's valid UTF-8)
//             if let Ok(body_text) = std::str::from_utf8(body) {
//                 println!("Raw Body: {}", body_text);
//                 match serde_json::from_str::<Value>(body_text) {
//                     Ok(parsed_json) => {
//                         println!("Parsed JSON: {}", parsed_json);
//                     }
//                     Err(err) => {
//                         println!("Failed to parse JSON: {}", err);
//                     }
//                 }
//             } else {
//                 println!("Body is not valid UTF-8");
//             }

//             // Send a response back
//             let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nGot it bro\n";
//             stream.write_all(response.as_bytes()).unwrap();
//         } else {
//             println!("Failed to parse HTTP request");
//         }
//     }
// }



// use std::{
    //     io::{prelude::*, BufReader},
    //     net::{TcpListener, TcpStream},
    // };
    
    // fn main() {
        //     let listener = TcpListener::bind("127.0.0.1:1604").unwrap();
        
        //     for stream in listener.incoming() {
            //         let stream = stream.unwrap();
            
            //         handle_connection(stream);
            //     }
            // }
            
            // fn handle_connection(mut stream: TcpStream) {
                //     let buf_reader = BufReader::new(&mut stream);
                //     let http_request: Vec<_> = buf_reader
                //         .lines()
                //         .map(|result| result.unwrap())
                //         .take_while(|line| !line.is_empty())
                //         .collect();
                
                //     let status_line = "HTTP/1.1 200 OK";
                //     let message = "Hello World!";
                //     let length = message.len();
                
                //     let response =
                //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{message}");
                
                //     stream.write_all(response.as_bytes()).unwrap();
                // }


// use http::{Request, Response, StatusCode};
// use std::io::Write;
// use std::net::TcpListener;

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();

//         // Create an HTTP response
//         let response = Response::builder()
//             .status(StatusCode::OK)
//             .header("Content-Type", "text/plain")
//             .body("Hello, world!")
//             .unwrap();

//         let response_string = format!(
//             "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
//             response.status(),
//             response.headers()
//                 .iter()
//                 .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap()))
//                 .collect::<Vec<_>>()
//                 .join("\r\n"),
//             response.body()
//         );

//         stream.write_all(response_string.as_bytes()).unwrap();
//     }
// }