mod route;

pub mod tsmwssh {
    use crate::route::Route;

    use std::{
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream}
    };
    
    pub struct ServerShell {
        listener: TcpListener,
        routes: Vec<Route>
    }
    
    impl ServerShell {
        pub fn new(port: u16, addr: &str) -> ServerShell {
            let server = ServerShell {
                listener: TcpListener::bind(&format!("{}:{}", addr, port)).unwrap(),
                routes: Vec::new(),
            };
            return server;
        }
    
        pub fn add_route<F>(&mut self, req_type: &str, url: &str, callback: F)
        where
            F: Fn(Vec<String>, TcpStream) + 'static,
        {
            self.routes.push(Route {
                req_type: req_type.to_string(),
                path: url.to_string(),
                callback: Box::new(callback),
            });
        }
    
        pub fn listen<F>(&self, listener_callback: F) 
        where 
            F: Fn(),
        {
            listener_callback();
        
            for stream in self.listener.incoming() {
                let mut stream: TcpStream = stream.unwrap();
            
                let buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<String> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();
            
                let req_data: Vec<&str> = http_request[0].split(" ").collect::<Vec<&str>>();
                let req_type: &str = req_data[0];
                let req_url: &str = req_data[1];
            
                for route in &self.routes {
                    if route.path == req_url && route.req_type == req_type {
                        (route.callback)(http_request, stream);
                        break;
                    }
                }
            }
        }
    }
}