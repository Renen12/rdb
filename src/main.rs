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
    parser::{Method, return_request_struct},
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
            Err(_) => String::from(""),
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let request = return_request_struct(unparsed.clone()).await;
    if request.method == Method::GET {
        println!("{}", request.path);
        let key_name = request.path.replace("/", "");
        let value = match get_value_from_key(key_name, database_path) {
            Some(v) => v,
            None => {
                match stream.shutdown(std::net::Shutdown::Read) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!("Cannot shutdown connection");
                    }
                }
                String::from("Invalid key name received")
            }
        };
        stream.write_all(value.as_bytes()).unwrap();
    }
}
