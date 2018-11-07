use futures::prelude::*;
use futures::future;
use std::thread;
use std::time;


fn main () {
    let future1 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(5));
        future::ok::<char, ()>('a')
    });

    let future2 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(3));
        future::ok::<char, ()>('b')
    });

    let (value, last_future) = future1.select(future2).wait().ok().unwrap();
    //assert_eq!(value, 'a');
    println!("{}", value);
    //assert_eq!(last_future.wait().unwrap(), 'b');
    println!("{}", last_future.wait().unwrap());
}
