use std::net::TcpStream;

pub struct Route {
    pub req_type: String,
    pub path: String,
    pub callback: Box<dyn Fn(Vec<String>, TcpStream)>,
}