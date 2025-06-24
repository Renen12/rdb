#[derive(Debug, PartialEq, PartialOrd)]
pub enum Method {
    GET,
    PATCH,
    POST,
    DELETE,
    UNDEFINED,
}

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: Method,
}
pub async fn return_request_struct(unparsed: Vec<String>) -> Request {
    let simple_request = &unparsed[0];
    let contents: Vec<_> = simple_request.split(" ").collect();
    let method = return_enum(contents[0]);
    return Request {
        path: contents[1].to_string(),
        method: method,
    };
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
