
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("created stream");
        io::write_all(stream, "hello, tokio\n").then(|result| {
            println!("wrote to stream; success={:?}", result.is_ok());
            Ok(())
        })
    })
    .map_err(|err| {
        println!("connection error = {:?}", err);
    });
    
    println!("start client.");
    tokio::run(client);
    println!("finish client.")

}
