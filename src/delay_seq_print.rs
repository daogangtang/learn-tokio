
use tokio::prelude::*;
use tokio::timer::Delay;

use std::time::{Duration, Instant};

fn main() {
    let when = Instant::now() + Duration::from_millis(2000);
    let task = Delay::new(when)
        .and_then(|_| {
            println!("Hello world!");
            Ok(())
        })
    .map_err(|e| panic!("delay errored; err={:?}", e));

    println!("started.");
    tokio::run(task);
}
