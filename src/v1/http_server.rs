// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
use std::{
    fs,
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};
// we can use crate:: to point the current project
use crate::util::thread::print_current_thread;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

/*
ref: https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview
* http request,
GET /echo HTTP/1.1
Host: reqbin.com
Accept: text/html

* http response, 
HTTP/1.1 200 OK
Date: Sat, 09 Oct 2010 14:28:02 GMT
Server: Apache
Last-Modified: Tue, 01 Dec 2009 20:18:22 GMT
ETag: "51142bc1-7449-479b075b2891b"
Accept-Ranges: bytes
Content-Length: 29769
Content-Type: text/html

<!DOCTYPE html>… (here come the 29769 bytes of the requested web page)
 */
 // get the 1st line of request.
 // buf_reader.lines().next() - Option<Result<std::string::String, std::io::Error>>
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