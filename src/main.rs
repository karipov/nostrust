// use httparse::Request;
// use std::io::{Read, Write};
// use std::net::TcpListener;

use httparse::{Request, EMPTY_HEADER};
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024]; // Allocate buffer for the incoming request

        // Read the incoming request into the buffer
        stream.read(&mut buffer).unwrap();

        // Create structures to parse the request line and headers
        let mut headers = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut headers);

        // Parse the HTTP request
        let result = request.parse(&buffer);
        if result.is_ok() {
            println!("Headers: {:?}", request.headers);
            
            // Extract the start of the body using the parsed headers
            let body_start = result.unwrap().unwrap();
            let body = &buffer[body_start..];

            // Convert body to a string (if it's valid UTF-8)
            if let Ok(body_text) = std::str::from_utf8(body) {
                println!("Body: {}", body_text);
            } else {
                println!("Body is not valid UTF-8");
            }

            // Send a response back
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            println!("Failed to parse HTTP request");
        }
    }
}


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