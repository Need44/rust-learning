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

fn get_route_from_r_uri(request_uri: &str) -> String{
    const URI_ROOT: &str = "frontend/";

    let routes = HashMap::from([
        ("/", "index"),
        ("/info", "info"),
    ]);

    // Safely look up the route, default to "404" if it doesnt exist
    let page = routes.get(request_uri).unwrap_or(&"404");

    format!("{}/{}.html", URI_ROOT, page)
}

fn handle_conn(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let request_line = (&mut buf_reader).lines().next().unwrap().unwrap();

    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let mut parts = request_line.split_whitespace();
    let request_type = parts.next();
    let request_uri = parts.next();
    
    //println!("Request: {http_request:#?}");
    
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let mut contents = fs::read_to_string(get_route_from_r_uri("error")).unwrap();
    
    //Some() handles possibility of null value if the value is actually present it unwraps it if not it doesnt execute
    if request_type == Some("GET"){
        if let Some(r_uri) = request_uri {
            let file_path = get_route_from_r_uri(r_uri);
        
            if r_uri == "/" || r_uri == "/info" {
                status_line = "HTTP/1.1 200 OK";
                contents = fs::read_to_string(file_path).unwrap();
            }
        }

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    //stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(response.as_bytes()).unwrap();

    }
}