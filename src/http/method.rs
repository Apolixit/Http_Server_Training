use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //Deux possibilités

        //1.
        // match s.parse::<Method>() {
        //     Ok(parsed_method) => { return Ok(parsed_method); },
        //     Err(_) => Err(MethodError),
        // }

        //2.
        // let try_parsed = s.parse::<Method>();
        // if try_parsed.is_err() {
        //     return Err(MethodError);
        // }

        // Ok(try_parsed.unwrap())

        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(MethodError)
        }
    }
}

#[derive(Debug)]
pub struct MethodError;