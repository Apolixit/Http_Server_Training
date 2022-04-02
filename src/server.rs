use std::{net::{TcpListener}, io::{Read}};
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Error while trying to convert the request : {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String
}

impl Server {
    pub fn new(address : String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler : impl Handler) {
        println!("Listening on {}", self.address);
        // let socketAddress = SocketAddr::from(SocketAddrV6::new() &self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        'listenerProgram: loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // let a = [1, 2, 3, 4, 5, 6, 7, 8];
                    // arr(&a[1..3]);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // println!("Youhou we got a message : {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Error when trying to send response : {}", e);
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection : {}", e),
                    }

                }
                Err(e) => {
                    println!("Client error {:?}, exit loop", e);
                    break 'listenerProgram;
                }
            }
        }
    }
}