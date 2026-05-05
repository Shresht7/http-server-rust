use std::io::{self, BufRead, Write};
use std::net;
use std::process;

use lib::http;

/// The network address host to listen on
const ADDRESS_HOST: &str = "127.0.0.1";

/// The network address port to listen on
const ADDRESS_PORT: u16 = 8080;

/// The main entry point of the application
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Runs the HTTP server
fn run() -> io::Result<()> {
    // Bind the TCP listener to the specified address and port
    let listener = net::TcpListener::bind((ADDRESS_HOST, ADDRESS_PORT))?;
    println!("Server Listening on: {}:{}", ADDRESS_HOST, ADDRESS_PORT);

    // Listen for incoming connections...
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())
}

fn handle_connection(mut stream: net::TcpStream) -> Result<(), io::Error> {
    let recv_addr = stream.local_addr()?;
    println!("Received connection from {}", recv_addr);

    let mut request_string = String::new();
    let reader = io::BufReader::new(&mut stream);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        request_string.push_str(&line);
        request_string.push('\n');
    }

    // Parse the HTTP request
    let request = request_string.parse::<http::Request>().map_err(|e| {
        eprintln!("Failed to parse request: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, "Failed to parse request")
    })?;

    println!("Parsed request: {:#?}", &request);

    // Generate a response based on the request URI
    let response = route(&request);

    // Send the response back to the client
    stream.write_all(&response.as_bytes())?;

    Ok(())
}

/// Routes the incoming HTTP request to the appropriate handler based on the request URI and generates an HTTP response.
fn route(request: &http::Request) -> http::Response {
    match request.uri.path.as_str() {
        "/" => handle_root(request),
        "/hello" => handle_hello(request),
        _ => handle_not_found(request),
    }
}

// --------------
// ROUTE HANLDERS
// --------------

fn handle_root(request: &http::Request) -> http::Response {
    http::Response::default()
        .header("Content-Type", "text/html")
        .body("<html><body><h1>Welcome to the Rust HTTP Server!</h1></body></html>")
}

fn handle_hello(request: &http::Request) -> http::Response {
    http::Response::default()
        .header("Content-Type", "text/html")
        .body("<html><body><h1>Hello, World!</h1></body></html>")
}

fn handle_not_found(request: &http::Request) -> http::Response {
    http::Response::default()
        .header("Content-Type", "text/html")
        .body("<html><body><h1>404 Not Found</h1></body></html>")
}
