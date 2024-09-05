mod v1;
mod v2;
mod util;
mod protocol;

use v2::http_server::DLHttpServer;
use crate::protocol::HttpServer;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // v1::http_server::run();
    let server = DLHttpServer::new("server");
    server.run(7878);
}
