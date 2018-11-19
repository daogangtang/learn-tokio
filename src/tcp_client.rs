
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::codec::BytesCodec;
//use std::io::{Read, Write};
use tokio::codec::Decoder;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let mut stdout = io::stdout();
    let client = TcpStream::connect(&addr)
    .and_then(|stream| {
        let framed = BytesCodec::new().framed(stream);
        let (sink, stream) = framed.split();

        let a = sink.send("hello, tokio\n".into())
            .and_then(|a_sink|{
                let a_stream = stream.map(|msg|{
                    println!("received msg: {:?}", msg);
                    "hello, io\n".into()
                });
                a_sink.send_all(a_stream)
            });

        a
    })
    .map(|_| ())
    .map_err(|err| {
        println!("connection error = {:?}", err);
    });
    
    println!("start client.");
    tokio::run(client);
    println!("finish client.")

}
