// This is web server project from https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
use std::{
    fs,
    collections::HashMap,
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
    let mut buf_reader = BufReader::new(&stream);
    let request_line = (&mut buf_reader).lines().next().unwrap().unwrap();

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    const URI_ROOT: &str = "frontend/";

    let mut parts = request_line.split_whitespace();
    let request_type = parts.next();
    let request_uri = parts.next();

    //println!("Request: {http_request:#?}");
    let routes = HashMap::from([
        ("/", "index"),
        ("/info", "info"),
        ("error", "404"),
    ]);

    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let mut contents = fs::read_to_string(format!("{}/{}.html", URI_ROOT, routes["error"])).unwrap();
    let mut length = contents.len();

    if request_type == Some("GET"){
        if let Some(r_uri) = request_uri{
                if let Some(&page) = routes.get(r_uri){
                    status_line = "HTTP/1.1 200 OK";
                    contents = fs::read_to_string(format!("{}/{}.html", URI_ROOT, page)).unwrap();
                    length = contents.len();
                }
        }   
    
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    //stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(response.as_bytes()).unwrap();

    }
}