use std::io::{self, Write};
use std::net;
use std::process;

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

/// Separator for HTTP response lines, consisting of a carriage return and line feed (`\r\n`)
const CRLF: &str = "\r\n";

fn handle_connection(mut stream: net::TcpStream) -> Result<(), io::Error> {
    let recv_addr = stream.local_addr()?;
    println!("Received connection from {}", recv_addr);

    // HTTP Response is made up of three parts, each separated by a [CRLF](https://developer.mozilla.org/en-US/docs/Glossary/CRLF) (`\r\n`):
    // 1. Status Line: Contains the HTTP version, status code, and reason phrase. Example: `HTTP/1.1 200 OK`
    // 2. Headers: Key-value pairs that provide additional information about the response. Example: `Content-Type: text/html`
    // 3. Body: (Optional) The actual content of the response, which can be HTML, JSON, or any other data format. Example: `<html><body><h1>Hello, World!</h1></body></html>`

    let status_line = "HTTP/1.1 200 OK\r\n";
    let headers = "Content-Type: text/html\r\n\r\n";
    let body = "<html><body><h1>Hello, World!</h1></body></html>";
    let response = format!("{status_line}{CRLF}{headers}{CRLF}{CRLF}{body}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}
