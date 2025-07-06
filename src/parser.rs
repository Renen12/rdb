#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Method {
    GET,
    PATCH,
    POST,
    DELETE,
    UNDEFINED,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub path: String,
    pub method: Method,
    pub headers: Vec<Vec<String>>,
}
pub fn undefined_request() -> Request {
    Request {
        path: String::from("_REQUEST_STRUCT_FAIL"),
        method: Method::UNDEFINED,
        headers: Vec::new(),
    }
}
pub fn return_request_struct(unparsed: Vec<String>) -> Option<Request> {
    let simple_request = match unparsed.get(0) {
        Some(v) => v,
        None => {
            eprintln!("Cannot read item of index 0 from {:?}", unparsed);
            return None;
        }
    };
    let contents: Vec<_> = simple_request.split(" ").collect();
    let method = return_enum(match contents.get(0) {
        Some(v) => v,
        None => {
            eprintln!("{:?} \n does not have a method", contents);
            return None;
        }
    });
    // Prepare for header registering
    let mut headers: Vec<Vec<String>> = Vec::new();
    let mut new = unparsed.clone();
    new.remove(0);
    for unparsed_header in new {
        let split: Vec<&str> = unparsed_header.split(":").collect();
        let header_name = split.get(0)?.to_string();
        let mut header_value = split.get(1)?.to_string();
        if header_value.starts_with(" ") {
            header_value.remove(0);
        }
        headers.push(vec![header_name, header_value]);
    }
    return Some(Request {
        path: match contents.get(1) {
            Some(v) => v,
            None => {
                return None;
            }
        }
        .to_string(),
        method: method,
        headers: headers,
    });
}
fn return_enum(string: &str) -> Method {
    if string == "GET" {
        return Method::GET;
    } else if string == "PATCH" {
        return Method::PATCH;
    } else if string == "POST" {
        return Method::POST;
    } else if string == "DELETE" {
        return Method::DELETE;
    }
    return Method::UNDEFINED;
}
