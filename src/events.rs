use std::{io::Write, net::TcpStream, sync::Mutex};

use rand::Rng;

use crate::{parser::Request, write_to_log_file_if_available};
#[derive(Debug)]
pub struct Subscription {
    pub stream: TcpStream,
    pub id: usize,
    name: String,
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
            let mut rng = rand::rng();
            let id: usize = rng.random_range(0..10000);
            return Some(Subscription {
                stream,
                id,
                name: event_name.clone(),
            });
        }
    }
    None
}
pub fn trigger_event(event_name: &String, subscriptions: &'static Mutex<Vec<Subscription>>) {
    println!("{:?}", subscriptions);
    for sub in subscriptions.lock().unwrap().iter_mut() {
        if sub.name == *event_name {
            sub.stream.write_all("triggered".as_bytes()).unwrap();
        }
    }
}
