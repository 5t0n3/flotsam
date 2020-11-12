use std::{
    collections::HashMap,
    io::{self, prelude::*, BufReader, BufWriter},
    net::TcpStream,
};

pub mod server;

// Chapter 6 handler (word/definition database)
pub fn process_word_request(
    stream: TcpStream,
    state: &mut HashMap<String, String>,
) -> io::Result<()> {
    let mut request: String = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);

    // Read lines (i.e. up to \n) of request until the end of it
    while let Ok(len) = reader.read_line(&mut request) {
        // Line doesn't just consist of \n
        if len > 1 {
            // Remove \n from the end of the request
            request.pop();

            // BufWriter to write to response
            let mut writer = BufWriter::new(stream.try_clone()?);

            let split_request: Vec<_> = request.split(" ").collect();

            // Match based on "verb" (command/request type)
            match split_request[0] {
                // Get a word from dictionary
                "GET" => {
                    let word = split_request[1];

                    // Respond with definition if it exists, or error if not
                    match state.get(word) {
                        Some(definition) => write!(writer, "ANSWER {}\n", definition)?,
                        None => write!(writer, "ERROR undefined word: {}\n", word)?,
                    };
                }

                // Add a word to dictionary
                "SET" => {
                    // Get word/definition from request
                    let word = split_request[1].to_owned();
                    let definition = split_request[2..].join(" ");

                    // Add word/definition combo to state (dictionary)
                    state.insert(word.clone(), definition);

                    // Let client know word was updated
                    write!(writer, "ANSWER definition updated for word {}\n", word)?
                }

                // Give all current definitions in dictionary
                "ALL" => {
                    let mut definitions = String::new();

                    for (word, definition) in state.iter() {
                        definitions.extend(format!("{}: {}\n", word, definition).chars());
                    }

                    if !definitions.is_empty() {
                        write!(
                            writer,
                            "ANSWER list of all current definitions:\n{}",
                            definitions
                        )?
                    } else {
                        write!(writer, "ANSWER no definitions currently in dictionary\n")?
                    }
                }

                // Clear all words currently in dictionary
                "CLEAR" => {
                    state.clear();

                    write!(writer, "ANSWER definitions successfully cleared.\n")?
                }

                // Fallback case for unknown command
                verb => write!(writer, "ERROR unknown command: {}\n", verb)?,
            }

            // Clear request string, as it is reused on each iteration
            request.clear();
        } else {
            return Ok(());
        }
    }

    Ok(())
}
