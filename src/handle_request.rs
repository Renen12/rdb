use std::{io::Write, net::TcpStream};

use crate::{
    database::{get_value_from_key, write_to_db},
    parser::{Method, Request},
    write_to_log_file_if_available,
};
pub fn handle_request(request: Request, mut stream: TcpStream, database_path: String) {
    println!("{}", request.path);
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
        let raw_path = request.path.replace("/", "");
        if !raw_path.contains("?") {
            write_to_log_file_if_available(format!(
                "No changes were supplied in POST request {:?}",
                request
            ));
            return;
        }
        let split: Vec<&str> = raw_path.split("?").collect();
        let changes_unparsed = match split.get(1) {
            Some(v) => v,
            None => {
                write_to_log_file_if_available("No valid changes were supplied".to_owned());
                return;
            }
        };
        let parsed_values: Vec<&str> = changes_unparsed.split("&").collect();
        for value in parsed_values {
            let key = value.split("=").collect::<Vec<&str>>()[0];
            let value = value.split("=").collect::<Vec<&str>>()[1];
            write_to_db(key.to_owned(), value.to_owned());
        }
    }
    if request.method == Method::UNDEFINED {
        eprintln!("Request is not valid.");
    }
}
