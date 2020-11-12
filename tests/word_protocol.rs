use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::thread;

use flotsam::process_word_request;
use flotsam::server::Server;

#[test]
fn set_and_get_definition() -> io::Result<()> {
    let mut server = Server::new("127.0.0.1:80", HashMap::new(), process_word_request)?;

    thread::spawn(move || server.run());
    let mut client = TcpStream::connect("127.0.0.1:80")?;

    let definition = "A procedure intended to establish the quality, performance, or reliability of something, especially before it is taken into widespread use.";

    write!(&mut client, "SET test {}\nGET test\n\n", definition)?;

    let mut reader = BufReader::new(client);

    // SET response
    let mut set_response = String::new();
    reader.read_line(&mut set_response)?;

    // Remove \n
    set_response.pop();

    assert_eq!(set_response, "ANSWER definition updated for word test");

    // GET response
    let mut get_response = String::new();
    reader.read_line(&mut get_response)?;

    // Remove \n
    get_response.pop();

    assert_eq!(get_response, "ANSWER A procedure intended to establish the quality, performance, or reliability of something, especially before it is taken into widespread use.");

    Ok(())
}
