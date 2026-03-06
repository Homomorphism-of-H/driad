use std::error::Error;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use tokio::io::AsyncWriteExt;
use tokio::net::{ TcpListener};

use driad_common::SharedEntity;
use hecs::World;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();

    world.spawn((SharedEntity,));

    let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7854)).await?;

    'accept_connections: loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("{addr}");
        stream.write_all(b">>> ").await?;
        drop(stream);
    }

    // Ok(())
}
