use std::io;
use std::net::{
    TcpListener
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Received connection!");
            },
            Err(error) => {
                println!("Error connecting: {}", error)
            }

        }
    }

    Ok(())
}
