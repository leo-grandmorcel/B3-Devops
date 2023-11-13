use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    collections::HashMap,
    env
};

fn main() {
    let port = match env::var("PING_LISTEN_PORT") {
        Ok(v) => v,
        Err(_e) => "8069".to_string()
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).unwrap();
    println!("Listening on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    if http_request[0] == "GET /ping HTTP/1.1"{
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in http_request.iter().skip(1) {
            let header = line.split(": ").collect::<Vec<&str>>();
            headers.insert(header[0].to_string(), header[1].to_string());
        }
        let contents = serde_json::to_string(&headers).unwrap().to_string();
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let response =
        format!("{status_line}{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }else{
        stream.write_all("HTTP/1.1 404 NOT FOUND".as_bytes()).unwrap();
    }
}