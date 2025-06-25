use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    database::get_value_from_key,
    parser::{Method, Request},
};
pub fn handle_request(request: Request, mut stream: TcpStream, database_path: String) {
    if request.method == Method::GET {
        let mut status_line = "HTTP/1.1 200 OK";
        let key_name = request.path.replace("/", "");
        let value = match get_value_from_key(&key_name, database_path) {
            Some(v) => v,
            None => {
                status_line = "HTTP/1.1 404 NOT FOUND";
                format!("Key {} not found", &key_name).to_owned()
            }
        };
        let value_length = value.len();
        let response = format!("{status_line}\r\nContent-Length: {value_length}\r\n\r\n{value}");
        println!("{response}");
        stream.write_all(response.as_bytes()).unwrap();
    }
    if request.method == Method::POST {
        let mut status_line = "HTTP/1.1 200 OK";
    }
    if request.method == Method::UNDEFINED {
        eprintln!("Request is not valid.");
    }
}
