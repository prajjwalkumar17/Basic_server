use super::{StatusCode, Request};
use std::io::{Write,Result as IOResult};
use std::{fmt::{Display,Formatter,Result as FmtResult, write}, net::TcpStream};
#[derive(Debug)]
pub struct Response{
    status_code:StatusCode,
    body:Option<String>,
}
impl Response{
pub fn new(status_code:StatusCode,body:Option<String>)->Self{
    Response { status_code, body }
}
pub fn send(&self,stream:&mut impl Write)->IOResult<()>{
    let body=match &self.body{
        Some(b)=>b,
        None => "",
    };
    write!(stream, "HTTP/1.1 {} {}\r\n\n{}",self.status_code,self.status_code.reason_phrase(),body)
}
}