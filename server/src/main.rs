use std::io::{self, BufReader};
use std::io::prelude::*;
use capnp::serialize_packed;
use std::error::Error;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

mod point_capnp {
    include!(concat!("generated/capnp", "/point_capnp.rs"));
}

const SERVER: Token = Token(0);

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Me server!");

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    println!("Server starting...");

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let (mut connection, _) = server.accept().unwrap();
                    println!("Server accepted!");

                    let mut reader = BufReader::new(connection);
                    let msg_reader = serialize_packed::read_message(&mut reader, ::capnp::message::ReaderOptions::new(),)?;

                    let msg = msg_reader.get_root::<point_capnp::point::Reader>()?;

                    println!("Got from client: ({}, {})", msg.get_x(), msg.get_y());
                    
                    drop(reader);
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}