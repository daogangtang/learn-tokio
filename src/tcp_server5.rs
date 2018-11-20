
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::env;
use std::net::SocketAddr;

use tokio::codec::BytesCodec;
use tokio::codec::Decoder;
use std::time::{Duration, Instant};
use tokio::timer::Delay;


fn main() {
    
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let task = socket.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            println!("start delay!");

            let when = Instant::now() + Duration::from_millis(1000);
            let a_task = Delay::new(when)
                .map_err(|e| println!("{:?}", e))
                .then(|_| {
                    io::write_all(socket, b"welcome, guy!")
                })
                .then(|_| Ok(()));

            tokio::spawn(a_task)
        });

    tokio::run(task);
}
