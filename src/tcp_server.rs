
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::env;
use std::net::SocketAddr;

fn main() {
    
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&add).unwrap();
    println!("Listening on: {}", addr);

    let task = socket.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        for_each(move |socket| {
            // fill biz here
            let (reader, writer) = socket.split();


            tokio::spawn()
        });

    tokio::run(task);
}
