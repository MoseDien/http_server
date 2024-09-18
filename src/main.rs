use http_server::v1;
use http_server::v2;
use http_server::warp;
use http_server::protocol::HttpServer;

#[tokio::main]
async fn main() {
    // v1
    // v1::http_server::run();

    // v2
    // let server = v2::http_server::DLHttpServer::new("DL");
    // server.run(3030);

    // warp
    warp::run().await;
}