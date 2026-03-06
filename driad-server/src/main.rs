use std::error::Error;
use std::io::Write;
use std::net::{Ipv4Addr, TcpListener};

use driad_common::SharedEntity;
use hecs::World;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();

    world.spawn((SharedEntity,));

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7854))?;

    'accept_connections: loop {
        let (mut stream, addr) = listener.accept()?;
        println!("{addr}");
        stream.write_all(b"Hello!")?;
        drop(stream);
    }

    // Ok(())
}
