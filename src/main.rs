use std::collections::HashMap;
use std::io;
use std::net::TcpListener;

use flotsam::{handle_connection, process_word_request};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    println!("Listening for connections!");

    let mut state = HashMap::new();

    // Listen for connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Received connection!");
                handle_connection(stream, &mut state, process_word_request)?;
            }
            Err(error) => eprintln!("Error accepting connection: {}", error),
        }
    }

    Ok(())
}
