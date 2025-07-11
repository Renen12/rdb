use std::{io::Write, net::TcpStream, sync::Mutex};

use crate::{parser::Request, write_to_log_file_if_available};
#[derive(Debug)]
pub struct Subscription {
    pub stream: TcpStream,
    name: String,
}
impl Drop for Subscription {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
    }
}
pub fn subscribe(request: Request, stream: TcpStream) -> Option<Subscription> {
    for pair in request.headers {
        if match pair.get(0) {
            Some(v) => v,
            None => {
                write_to_log_file_if_available("Cannot get header name".to_owned());
                return None;
            }
        } == &String::from("Event-Name")
        {
            // Run if the header is Event-Name
            let event_name = &pair[1];
            return Some(Subscription {
                stream,
                name: event_name.clone(),
            });
        }
    }
    None
}
pub fn trigger_event(
    event_name: &String,
    subscriptions: &'static Mutex<Vec<Subscription>>,
    response_to_subscribers: &String,
) {
    for sub in subscriptions.lock().unwrap().iter_mut() {
        if sub.name == *event_name {
            let _ = sub.stream
                .write_all(response_to_subscribers.as_bytes()).inspect_err(|_| {
                    write_to_log_file_if_available(String::from("\nCannot write to subscriber stream"));
                });
        }
    }
}
