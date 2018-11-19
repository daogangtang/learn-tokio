use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;

fn main() {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<std::net::SocketAddr>().unwrap();

    let socket = tokio::net::TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let task = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = tokio::codec::Framed::new(socket, tokio::codec::BytesCodec::new());
            let (sink, stream) = framed.split();

            let a_stream = stream
                .map(|bytes| {
                    println!("bytes: {:?}", bytes);
                    let when = std::time::Instant::now() + std::time::Duration::from_millis(1000);
                    tokio::timer::Delay::new(when)
                })
            .map(|_| {
                println!("start delay!");
                "welcome, guy!".into()
            })
            .map_err(|e| { println!("delay errored; err={:?}", e); e});

            let a = sink
                .send_all(a_stream)
                .map(|_| ())
                .map_err(|e| println!("==> {:?}", e));

            tokio::spawn(a)
        });

    tokio::run(task);
}

