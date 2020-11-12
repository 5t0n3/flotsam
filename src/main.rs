use std::collections::HashMap;
use std::io;

use flotsam::server::Server;
use flotsam::{handle_connection, process_word_request};

fn main() -> io::Result<()> {
    println!("Listening for connections!");

    let mut server = Server::new("127.0.0.1:80", HashMap::new(), |stream, state| {
        handle_connection(stream, state, process_word_request)
    })?;

    server.run()?;

    Ok(())
}
