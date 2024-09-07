use warp::{Filter, Rejection, Reply};
use std::convert::Infallible;

async fn handle_404() -> Result<impl Reply, Infallible> {
    Ok(warp::reply::html("<h1>404 - Not Found</h1><p>Welcome to unknown world!</p>"))
}
    
#[tokio::main]
async fn main() {
    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

    
    let routes = warp::get().and(
        hello_world
        .or(hello)
        .or(hi)
        .or(warp::any().and_then(handle_404))
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}