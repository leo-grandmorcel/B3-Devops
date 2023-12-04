use regex::Regex;
use gethostname::gethostname;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    env,
    thread,
};

fn main() {
    let address = match env::var("PING_LISTEN_PORT") {
        Ok(value) => format!("0.0.0.0:{}",value),
        Err(_err) => format!("0.0.0.0:8080"),
    };
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Server listening : {:#?}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Spawn a new thread for each connection
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Hostname: {:?}", gethostname());
    let ping_regex = Regex::new(r"^GET \/ping ").unwrap();
    if !request.is_empty(){
        println!("REQUEST : {}", request[0]);
    }
    if !request.is_empty() && ping_regex.is_match(&request[0]) {
        println!("REQUEST : {}", request[0]);
        let mut response_body : Vec<String> = Vec::new();
        for data in request[1..].into_iter() {
            let (key, value) = data.split_once(": ").unwrap();
            let value = value.trim().replace("\"", "");
            if value.contains(",") {
                let values: Vec<&str> = value.split(",").collect();
                let mut values_list: Vec<String> = Vec::new();
                for v in values.into_iter() {
                    let v = v.trim();
                    values_list.push(format!("\"{}\"", v));
                }
                response_body.push(format!("\"{}\": [{}]", key, values_list.join(", ")));
            } else {
                response_body.push(format!("\"{}\": \"{}\"", key, value));
            }
        }
        let response = format!("HTTP/1.1 200 OK\r\n");
        let response = format!("{response}Content-Type: application/json\r\n");
        let response = format!("{response}\r\n{{{}}}", response_body.join(", "));
        match stream.write_all(response.as_bytes()) {
            Ok(_r) => (),
            Err(err) => println!("{err}"),
        }
    } else {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n");
        match stream.write_all(response.as_bytes()) {
            Ok(_r) => (),
            Err(err) => println!("{err}"),
        }
    }
}