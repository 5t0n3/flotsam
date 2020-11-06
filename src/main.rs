use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    net::TcpListener,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    println!("Listening for connections!");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Received connection!");

                // Read message from client, line by line
                let mut reader = BufReader::new(stream);
                let mut msg_buf = String::new();

                while let Ok(num) = reader.read_line(&mut msg_buf) {
                    // Break on empty line (\r\n)
                    if num <= 2 {
                        break;
                    }
                }

                // Dispose of reader after reading entire message
                stream = reader.into_inner();

                // Create a BufWriter to write to the stream (response)
                let mut response = BufWriter::new(stream);

                // Hello message
                response.write_all(b"Hello from flotsam! Your message was:\r\n")?;

                // Respond with message back to client
                response.write_all(msg_buf.as_bytes())?;
            }
            Err(error) => eprintln!("Error accepting connection: {}", error),
        }
    }

    Ok(())
}
