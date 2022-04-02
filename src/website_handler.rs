use crate::http::{Response, StatusCode, Method};
use std::fs;
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        let path_borrowed = &path[..];

        let canonicalized_public_path = fs::canonicalize(&self.public_path).unwrap();

        match fs::canonicalize(path_borrowed) {
            Ok(path_buf) => {
                println!("\tpath_buf = {:?}\n\tpublic path = {}\n\tcanonicalize public path = {:?}", path_buf, self.public_path, canonicalized_public_path);
                if path_buf.starts_with(canonicalized_public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Hacking !!!! {}", file_path);
                    None
                }
            }
            Err(_) => None,

        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(file_content) => Response::new(StatusCode::OK, Some(file_content)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}