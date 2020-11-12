use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server<S, H> {
    listener: TcpListener,
    state: S,
    handler: H,
}

impl<S, H> Server<S, H>
where
    H: FnOnce(TcpStream, &mut S) -> io::Result<()> + Copy,
{
    pub fn new<A: ToSocketAddrs>(addr: A, init_state: S, handler: H) -> io::Result<Server<S, H>> {
        let listener = TcpListener::bind(addr)?;

        Ok(Server {
            listener: listener,
            state: init_state,
            handler: handler,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Received connection!");
                    (self.handler)(stream, &mut self.state)?;
                }
                Err(error) => eprintln!("Error accepting connection: {}", error),
            }
        }

        Ok(())
    }
}
