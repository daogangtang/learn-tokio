
use tokio::prelude::*;
use tokio::timer::Interval;
use futures::sync::mpsc;
use std::time::{Duration, Instant};
use std::io;
use futures::try_ready;


/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<Vec<u8>>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<Vec<u8>>;

struct Server {
    rx0: Rx,
    tx1: Tx
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            //let msg = try_ready!(self.rx0.poll().unwrap());
            match self.rx0.poll().unwrap() {
                Async::Ready(Some(msg)) => {
                    println!("--> {:?}", msg);
                    self.tx1.unbounded_send(msg).unwrap();
                }
                _ => break,
            }

        }
        Ok(Async::NotReady)
    }
}

fn main() {

    let (tx0, rx0): (Tx, Rx) = mpsc::unbounded();
    let (tx1, mut rx1): (Tx, Rx) = mpsc::unbounded();

    let task = Interval::new(Instant::now(), Duration::from_millis(1000))
        //.map(move |x| (x, tx0.clone()))
        //.for_each(|(_instant, tx)| {
        .for_each(move |_instant| {
            //let tx = tx0.clone();
            tx0.unbounded_send(b"test".to_vec()).unwrap();
            println!("---> instant={:?}", _instant);

            match rx1.poll().unwrap() {
                Async::Ready(Some(msg)) => {
                    println!("<=== {:?}", msg);
                }
                _ => (),
            }

            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    let server = Server {
        rx0: rx0,
        tx1: tx1
    }
    .map_err(|e| println!("errored; err={:?}", e));

    // could not spawn task here
    //let task2 = tokio::spawn(server);


    let tasks = server.join(task)
        .and_then(|_|{
            Ok(())
        });

    tokio::run(tasks);
}
