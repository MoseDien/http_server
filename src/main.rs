use warp::{Filter, Reply};
use std::convert::Infallible;

async fn handle_404() -> Result<impl Reply, Infallible> {
    Ok(warp::reply::html("<h1>404 - Not Found</h1><p>Welcome to unknown world!</p>"))
}

#[tokio::main]
async fn main() {
    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /rust
    let hi = warp::path("rust").map(|| "Hello, Rust!");

    // GET /rusp/warp => 200 OK with body "Rust, warp!"
    let hello = warp::path!("rust" / String)
        .map(|name| format!("Rust, {}!", name));
    // 添加访问日志功能
    let log = warp::log::custom(|info| {
        println!("{} {} {} {}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed().as_millis()
        );
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
