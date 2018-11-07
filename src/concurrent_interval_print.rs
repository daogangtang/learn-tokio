
use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

fn main() {
    let task = Interval::new(Instant::now(), Duration::from_millis(1000))
        .for_each(|instant| {
            println!("---> instant={:?}", instant);
            Ok(())
        })
    .map_err(|e| panic!("interval errored; err={:?}", e));

    let task2 = Interval::new(Instant::now(), Duration::from_millis(1507))
        .for_each(|instant| {
            println!("<======= instant={:?}", instant);
            Ok(())
        })
    .map_err(|e| panic!("interval errored; err={:?}", e));

    let tasks = task.join(task2)
        .and_then(|_|{
            Ok(())
        });

    tokio::run(tasks);
}
