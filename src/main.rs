use std::io::{self, Write};
use std::net;
use std::process;

mod constants;
mod response;

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

    // Create a simple HTTP response
    let response = response::Response::default()
        .header("Content-Type", "text/html")
        .body("<html><body><h1>Hello, World!</h1></body></html>");

    // Send the response back to the client
    stream.write_all(&response.as_bytes())?;

    Ok(())
}
