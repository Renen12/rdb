use std::{
    env::temp_dir,
    fs,
    io::Write,
    net::TcpStream,
    sync::{Mutex, OnceLock},
};

use crate::{
    database::{get_db, get_value_from_key, write_to_db},
    events::{self, Subscription, subscribe, trigger_event},
    get_database_path,
    parser::{Method, Request},
    write_to_log_file_if_available,
};
static SUBSCRIPTIONS: OnceLock<Mutex<Vec<Subscription>>> = OnceLock::new();

fn get_subscriptions() -> &'static Mutex<Vec<Subscription>> {
    SUBSCRIPTIONS.get_or_init(|| Mutex::new(Vec::new()))
}
pub fn handle_request(request: Request, mut stream: TcpStream, database_path: String) {
    // Subscription handling
    if request.path == "/subscribe" && request.method == Method::POST {
        let v = subscribe(request, stream).unwrap();
        get_subscriptions().lock().unwrap().push(v);
        return;
    }
    if request.path == "/trigger" && request.method == Method::POST {
        for pair in request.headers {
            if pair[0] == "Event-Name" {
                trigger_event(&pair[1], get_subscriptions());
            }
        }

        return;
    }
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
        stream.write_all(response.as_bytes()).unwrap();
    }
    if request.method == Method::POST {
        let mut _status_line = "HTTP/1.1 200 OK";
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
            let split = value.split("=").collect::<Vec<&str>>();
            let key = match split.get(0) {
                Some(v) => v,
                None => {
                    return;
                }
            }
            .to_owned();
            let value = match split.get(1) {
                Some(v) => v,
                None => {
                    return;
                }
            }
            .to_owned();
            write_to_db(key.to_owned(), value.to_owned());
        }
    }
    if request.method == Method::DELETE {
        let key = request.path.replacen("/", "", 1);
        let mut line_c = 0;
        let base = get_db(get_database_path());
        let split = base.split("\n");
        let mut db_pairs = split.collect::<Vec<&str>>();
        let mut found = false;
        let mut status_line = "HTTP/1.1 204 No Content";
        for line in db_pairs.clone() {
            let split: &Vec<&str> = &line.split("=").collect();
            let key_in_db = match split.get(0) {
                Some(v) => v,
                None => {
                    return;
                }
            };
            if *key_in_db.to_owned() == key {
                found = true;
                break;
            }
            line_c += 1;
        }
        if found {
            db_pairs.remove(line_c);
        } else {
            status_line = "HTTP/1.1 404 Not Found";
        }
        let mut final_string = String::new();
        for item in db_pairs {
            final_string.push_str(&(item.to_owned() + "\n"));
        }
        fs::write(get_database_path(), final_string).unwrap();
        let response = format!("{status_line}\r\n\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
    }
    if request.method == Method::UNDEFINED {
        eprintln!("Request is not valid.");
    }
}
