use capnp::serialize_packed;
use std::error::Error;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

mod point_capnp {
    include!(concat!("generated/capnp", "/point_capnp.rs"));
}

const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>> {
    let mut message = ::capnp::message::Builder::new_default();
    let mut point = message.init_root::<point_capnp::point::Builder>();
    point.set_x(10.0f32);
    point.set_y(12.4f32);

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:13265".parse()?;
    let mut client = TcpStream::connect(addr)?;
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;


    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                        serialize_packed::write_message(&mut client, &message)?;
                        println!("Client wrote!");
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}
