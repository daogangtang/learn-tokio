//! A "print-each-packet" server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! put down in the stdout everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example print\_each\_packet
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:8080
//!
//! Each line you type in to the `connect` terminal should be written to terminal!
//!
//! Minimal js example:
//!
//! ```js
//! var net = require("net");
//!
//! var listenPort = 8080;
//!
//! var server = net.createServer(function (socket) {
//!     socket.on("data", function (bytes) {
//!         console.log("bytes", bytes);
//!     });
//!
//!     socket.on("end", function() {
//!         console.log("Socket received FIN packet and closed connection");
//!     });
//!     socket.on("error", function (error) {
//!         console.log("Socket closed with error", error);
//!     });
//!
//!     socket.on("close", function (with_error) {
//!         if (with_error) {
//!             console.log("Socket closed with result: Err(SomeError)");
//!         } else {
//!             console.log("Socket closed with result: Ok(())");
//!         }
//!     });
//!
//! });
//!
//! server.listen(listenPort);
//!
//! console.log("Listening on:", listenPort);
//! ```
//!

#![deny(warnings)]

extern crate tokio;
extern crate tokio_codec;

use tokio_codec::BytesCodec;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::codec::Decoder;

use std::env;
use std::net::SocketAddr;

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = BytesCodec::new().framed(socket);
            //let (_writer, reader) = framed.split();
            // or let (sender, receiver) = framed.split();
            let (_writer, reader) = framed.split();

            let processor = reader
                .for_each(|bytes| {
                    println!("bytes: {:?}", bytes);
                    Ok(())
                })
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    // We have to return the error to catch it in the next ``.then` call
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });

            tokio::spawn(processor)
        });

    tokio::run(done);
}
