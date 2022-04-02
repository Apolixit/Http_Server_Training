use std::{fmt::Display, fmt::Formatter, fmt::Result as FmtResult};
use std::{io::{Result as IoResult, Write as IOWrite}};

use chrono::prelude::*;
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    datetime: DateTime<Local>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body, datetime : Local::now() }
    }

    pub fn send(&self, stream: &mut impl IOWrite) -> IoResult<()>{
        let body = match &self.body {
            Some(s) => s,
            None => ""
        };

        write!( stream,
                "HTTP/1.1 {} {}\r\n\n{}",
                self.status_code,
                self.status_code.reason_phrase(),
                body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        let body = match &self.body {
            Some(s) => s,
            None => ""
        };

        write!( f,
                "HTTP/1.1 {} {}\r\n\n{}",
                self.status_code,
                self.status_code.reason_phrase(),
                body)
    }
}