mod database;
mod events;
mod handle_request;
mod parser;
use std::{
    env::args,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};
mod threadpool;
use crate::{
    parser::{return_request_struct, undefined_request},
    threadpool::ThreadPool,
};
pub fn return_log_file() -> Option<File> {
    let mut options = OpenOptions::new();
    options.create(true);
    options.append(true);
    match options.open("rdb.log") {
        Ok(v) => return Some(v),
        Err(_) => {
            return None;
        }
    }
}
pub fn write_to_log_file_if_available(message: String) {
    return_log_file().inspect(|mut v| {
        let _ = v.write(message.as_bytes());
        return;
    });
}
pub fn get_database_path() -> String {
    let args: Vec<String> = args().collect();
    for arg in args {
        if arg.contains("--db") {
            let mut result = String::from(match arg.split("=").collect::<Vec<&str>>().get(1) {
                Some(v) => v,
                None => {
                    eprintln!("Failed to read db path from args, falling back to default");
                    "database.rdb"
                }
            });
            if result.is_empty() {
                result = "database.rdb".to_owned();
            }
            return result;
        }
    }
    return String::from("database.rdb");
}
static HELP_MESSAGE: &'static str = "--db=database.rdb — specify the database path";

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    for arg in args {
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
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            handle_connection(stream, &get_database_path());
        });
    }
}
fn handle_connection(stream: TcpStream, database_path: &str) {
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

    let request = match return_request_struct(unparsed.clone()) {
        Some(v) => v,
        None => {
            eprintln!("Cannot build request struct");
            undefined_request()
        }
    };
    return_log_file().inspect(|mut v| {
        v.write(format!("Request: {:?}", unparsed).as_bytes())
            .unwrap();
    });
    handle_request::handle_request(request, stream, database_path.to_owned());
}
