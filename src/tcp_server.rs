
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
            let framed = BytesCodec::new().framed(socket);
            let (sink, stream) = framed.split();

            let a_stream = stream.map(|bytes|{
                println!("bytes: {:?}", bytes);
                let when = Instant::now() + Duration::from_millis(1000);
                let a_task = Delay::new(when)
                    .and_then(|_| {
                        println!("start delay!");
                        Ok("welcome, guy!".into())
                    })
                    .map_err(|e| {println!("delay errored; err={:?}", e); e});
            
                a_task
            })
            .map(|_| ())
            .map_err(|e| println!("{:?}", e));

            let a = sink.send_all(a_stream)
                         .map(|_| ())
                         .map_err(|e| println!("==> {:?}", e));

            tokio::spawn(a)
        });

    tokio::run(task);
}
