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
}
pub async fn return_request_struct(unparsed: Vec<String>) -> Option<Request> {
    let simple_request = match unparsed.get(0) {
        Some(v) => v,
        None => {
            eprintln!("Cannot read item of index 0 from {:?}", &unparsed);
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
    return Some(Request {
        path: contents[1].to_string(),
        method: method,
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
