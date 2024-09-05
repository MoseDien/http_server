pub trait HttpServer {
    fn run(&self, port: i32);
}