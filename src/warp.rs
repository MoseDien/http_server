use warp::{Filter, Reply};
use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};


pub async fn run() {
    main().await;
}

async fn handle_404() -> Result<impl Reply, Infallible> {
    Ok(warp::reply::html("<h1>404 - Not Found</h1><p>Welcome to unknown world!</p>"))
}

async fn main() {
    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /rust
    let hi = warp::path("rust").map(|| "Hello, Rust!");

    // GET /rusp/warp => 200 OK with body "Rust, warp!"
    let hello = warp::path!("rust" / String)
        .map(|name| format!("Rust, {}!", name));

    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")
            .expect("cannot open to write file")
    ));

    let log = warp::log::custom(move |info| {
        let log_entry = format!("{} {} {} {}\n",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed().as_millis()
        );
        println!("{}", log_entry);
        let mut file = log_file.lock().unwrap();
        if let Err(e) = file.write_all(log_entry.as_bytes()) {
            eprintln!("failed to write to file: {}", e);
        }
    });

    let routes = warp::get()
        .and(
            hello_world
                .or(hello)
                .or(hi)
                .or(warp::any().and_then(handle_404))
        )
        .with(log);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}