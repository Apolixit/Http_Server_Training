use super::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::{QueryString};


//GET /search?name=abc&sort=1 HTTP/1.1
//path: /search
//query_string: name=abc&sort=1
//method = Method::GET
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl<'buf> Request<'buf> {
    pub fn new(path: &'buf str, query_string: Option<QueryString<'buf>>, method: super::method::Method) -> Request<'buf> {
        Self { path, query_string, method }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(value) {
        //     Ok(request) => {
        //     },
        //     Err(e) => return Err(ParseError::InvalidEncoding),
        // }
        // let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(value)?;

        // match get_next_word(request) {
        //     Some((method, request)) => {
        //     }
        //     None => return Err(ParseError::InvalidRequest),
        // }
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method_enum : Method = method.parse()?;
        let mut query_string = None;
        //First version
        // match path.find('?')  {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     },
        //     None => {},
        // }

        //Second version
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        //Third version
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method : method_enum
          })
    }
}

fn get_next_word<'a>(request: &'a str) -> Option<(&'a str, &'a str)> {
    // let mut chars = request.chars();
    // loop {
    //     let item = chars.next();
    //     match item {
    //         Some(c) => {
    //             // Do something
    //          },
    //         None => break,
    //     }
    // }
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidRequest => "Invalid request"
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError { }
