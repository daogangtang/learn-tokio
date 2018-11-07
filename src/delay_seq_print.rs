
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
        .and_then(|_| {
            let when = Instant::now() + Duration::from_millis(2000);
            Delay::new(when)
            .and_then(|_|{
                println!("Hello world 2!");
                Ok(())
            })
            .and_then(|_|{
                let when = Instant::now() + Duration::from_millis(2000);
                Delay::new(when)
                .and_then(|_|{
                    println!("Hello world 3!");
                    Ok(())
                })
            })
        })
        .map_err(|e| panic!("delay errored; err={:?}", e));

    println!("started.");
    tokio::run(task);
}
