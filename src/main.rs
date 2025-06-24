mod parser;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};

use crate::parser::return_request_struct;
#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7950") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Can't bind to port 7950: {}", e);
            exit(1);
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).await;
    }
}
async fn handle_connection(mut stream: TcpStream) {
    let unparsed: Vec<_> = BufReader::new(&stream)
        .lines()
        .map(|r| match r {
            Ok(v) => v,
            Err(_) => String::from(""),
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let request = return_request_struct(unparsed.clone()).await;
}
