// This is web server project from https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection established!");
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    //println!("Request: {http_request:#?}");
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("frontend/index.html").unwrap();
    let length = contents.len();
    
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    //stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(response.as_bytes()).unwrap();

}
