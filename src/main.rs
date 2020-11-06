use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

fn handle_connection<H>(stream: TcpStream, handler: H) -> io::Result<()>
where
    H: FnOnce(String, BufWriter<TcpStream>) -> io::Result<()>,
{
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
    let write_stream = reader.into_inner();

    // Create a BufWriter to write to the stream (response)
    let response = BufWriter::new(write_stream);

    // Pass control to handler function
    handler(msg_buf, response)?;

    Ok(())
}

// Chapter 4 handler
fn respond_with_message(message: String, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    // Hello message
    writer.write_all(b"Hello from flotsam! Your message was:\r\n")?;

    // Respond with message back to client
    writer.write_all(message.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    println!("Listening for connections!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Received connection!");
                handle_connection(stream, respond_with_message)?;
            }
            Err(error) => eprintln!("Error accepting connection: {}", error),
        }
    }

    Ok(())
}
