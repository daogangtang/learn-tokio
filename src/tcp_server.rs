use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;

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
    let addr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let task = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = BytesCodec::new().framed(socket);
            let (sink, stream) = framed.split();

            let a_stream = stream.and_then(|bytes| {
                println!("bytes: {:?}", bytes);
                println!("start delay!");
                Delay::new(Instant::now() + Duration::from_millis(1000))
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
                    .and_then(|_|{
                        Ok("welcome, guy!".into())
                    })
            });

            let a = sink.send_all(a_stream)
                    .map(|_| ())
                    .map_err(|e| println!("==> {:?}", e));

            tokio::spawn(a)
        });

    tokio::run(task);
}

