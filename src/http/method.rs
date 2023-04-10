use std::str::FromStr;
#[derive(Debug)]
pub enum Method{ 
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
impl FromStr for Method {
    type Err = method_error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET"=>Ok(Method::GET),
            "DELETE"=>Ok(Method:: DELETE),
            "POST"=>Ok(Method:: POST),
            "PUT"=>Ok(Method:: PUT),
            "HEAD"=>Ok(Method:: HEAD),
            "CONNECT"=>Ok(Method:: CONNECT),
            "OPTIONS"=>Ok(Method:: OPTIONS),
            "TRACE"=>Ok(Method::TRACE),
            "PATCH"=>Ok(Method:: PATCH),
            _=> Err(method_error)
        }
    }
}

pub struct method_error;