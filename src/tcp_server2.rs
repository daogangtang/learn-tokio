
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::env;
use std::net::SocketAddr;

use tokio::codec::BytesCodec;
use tokio::codec::Decoder;
use std::time::{Duration, Instant};

fn main() {
    
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let task = socket.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = BytesCodec::new().framed(socket);
            let (sink, stream) = framed.split();

            let a_stream = stream.map(|bytes| {
                println!("bytes: {:?}", bytes);
                "welcome, guy!".into()
            });
            let a = sink.send_all(a_stream);

            tokio::spawn(a
                         .map(|_| ())
                         .map_err(|e| println!("==> {:?}", e)))
        });

    tokio::run(task);
}
