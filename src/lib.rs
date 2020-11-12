use std::{
    collections::HashMap,
    io::{self, prelude::*, BufReader, BufWriter},
    net::TcpStream,
};

pub mod server;

// Generic connection handler function
pub fn handle_connection<H, K, V>(
    stream: TcpStream,
    state: &mut HashMap<K, V>,
    handler: H,
) -> io::Result<()>
where
    H: FnOnce(String, &mut HashMap<K, V>, &mut BufWriter<TcpStream>) -> io::Result<()>,
{
    // Read message from client, line by line
    let mut reader = BufReader::new(stream);
    let mut msg_buf = String::new();

    while let Ok(len) = reader.read_line(&mut msg_buf) {
        // Break on empty line (\r\n)
        if len <= 2 {
            break;
        }
    }

    // Dispose of reader after reading entire message into msg_buf
    let write_stream = reader.into_inner();

    // Create a BufWriter to write to the stream (response)
    let mut response = BufWriter::new(write_stream);

    // Pass control to handler function
    match handler(msg_buf, state, &mut response) {
        Ok(()) => Ok(()),
        Err(error) => write!(&mut response, "ERROR command unable to execute: {}", error),
    }
}

// Chapter 6 handler (word/definition database)
pub fn process_word_request(
    request: String,
    state: &mut HashMap<String, String>,
    writer: &mut BufWriter<TcpStream>,
) -> io::Result<()> {
    let split_request: Vec<_> = request
        // Split at spaces
        .split(" ")
        // Remove \r and \n (and any other non-whitespace characters) from input
        .map(|word| {
            word.to_owned()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        })
        .collect();

    // Match based on "verb" (command/request type)
    match &*split_request[0] {
        // Get a word from dictionary
        "GET" => {
            let word = &split_request[1];

            // Respond with definition if it exists, or error if not
            match state.get(word) {
                Some(definition) => write!(writer, "ANSWER {}", definition),
                None => write!(writer, "ERROR undefined word: {}", word),
            }
        }

        // Add a word to dictionary
        "SET" => {
            // Get word/definition from request
            let word = split_request[1].to_owned();
            let definition = split_request[2..].join(" ");

            // Add word/definition combo to state (dictionary)
            state.insert(word.clone(), definition);

            // Let client know word was updated
            write!(writer, "ANSWER definition updated for word {}", word)
        }

        // Give all current definitions in dictionary
        "ALL" => {
            let mut definitions = String::new();

            for (word, definition) in state.iter() {
                definitions.extend(format!("{}: {}\r\n", word, definition).chars());
            }

            if !definitions.is_empty() {
                write!(
                    writer,
                    "ANSWER list of all current definitions:\r\n{}",
                    definitions
                )
            } else {
                write!(writer, "ANSWER no definitions currently in dictionary")
            }
        }

        // Clear all words currently in dictionary
        "CLEAR" => {
            state.clear();

            write!(writer, "ANSWER definitions successfully cleared.")
        }

        // Fallback case for unknown command
        verb => write!(writer, "ERROR unknown command: {}", verb),
    }
}