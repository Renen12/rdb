mod database;
mod parser;
use std::{
    env::args,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};

use crate::{
    database::get_value_from_key,
    parser::{Method, Request, return_request_struct},
};
static HELP_MESSAGE: &'static str = "--db=database.rdb — specify the database path";
#[tokio::main]
async fn main() {
    let mut database_path = String::from("database.rdb");
    let args: Vec<String> = args().collect();
    for arg in args {
        if arg.contains("--db") {
            database_path = String::from(arg.split("=").collect::<Vec<&str>>()[1]);
        }
        if arg == "--help" {
            println!("{}", HELP_MESSAGE);
            exit(0);
        }
    }
    let listener = match TcpListener::bind("127.0.0.1:7950") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Can't bind to port 7950: {}", e);
            exit(1);
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, database_path.clone()).await;
    }
}
async fn handle_connection(mut stream: TcpStream, database_path: String) {
    let unparsed: Vec<_> = BufReader::new(&stream)
        .lines()
        .map(|r| match r {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Cannot read from stream");
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let request = match return_request_struct(unparsed.clone()).await {
        Some(v) => v,
        None => {
            eprintln!("Cannot build request struct");
            Request {
                path: "_REQUEST_STRUCT_FAIL".to_owned(),
                method: Method::UNDEFINED,
            }
        }
    };
    if request.method == Method::GET {
        let mut status_line = "HTTP/1.1 200 OK\r\n\r\n";
        println!("{}", request.path);
        let key_name = request.path.replace("/", "");
        let value = match get_value_from_key(key_name, database_path) {
            Some(v) => v,
            None => {
                status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                String::from("Key name not found")
            }
        };
        let value_length = value.len();
        let response = format!("{status_line}\r\n\r\n\r\n {value}");
        stream.write_all(response.as_bytes()).unwrap();
    }
    if request.method == Method::UNDEFINED {
        eprintln!("Request is not valid.");
    }
}
