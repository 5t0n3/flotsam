use std::{
    io::{self, Write},
    net::TcpListener,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Received connection!");

                if let Ok(addr) = stream.peer_addr() {
                    let msg_string = format!("Hello from flotsam! Your IP address is {}.", addr);
                    stream.write_all(msg_string.as_bytes())?;
                } else {
                    stream.write_all(b"Hello from flotsam!")?;
                }
            }
            Err(error) => eprintln!("Error accepting connection: {}", error),
        }
    }

    Ok(())
}
