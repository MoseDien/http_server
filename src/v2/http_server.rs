// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
use std::{
    fs,
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};
use threadpool::ThreadPool;
// we can use crate:: to point the current project
use crate::util::thread::print_current_thread;
use crate::protocol::HttpServer;

pub struct DLHttpServer {
    name: &'static str,
}

impl HttpServer for DLHttpServer {
    fn run(&self, port: i32) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&address).unwrap();
        let pool = ThreadPool::new(5);
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || {
                handle_connection(stream);
            });
        }
    }
}

impl DLHttpServer {
    pub fn new(name: &'static str) -> Self {
        DLHttpServer { name: name }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET /index.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/pub/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/pub/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    let useBuf = false;
    if useBuf {
        // 1- use stream to write
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // 2- use BufWriter `fn write_all(&mut self, buf: &[u8]) -> Result<usize>`
        let mut buf_writer = BufWriter::new(stream);
        buf_writer.write_all(response.as_bytes());
    }

    print_current_thread();
}