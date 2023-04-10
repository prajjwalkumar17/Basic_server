use super::method::{Method,method_error};
use super::QueryString;
use std::str::Utf8Error;
use std::str;
use std::error::Error;
use std::convert::TryFrom;
use std::fmt::{Display,Result as FmtResult,Formatter,Debug};
#[derive(Debug)]
pub struct Request<'buf>{
    path:&'buf str,
    query_string:Option<QueryString<'buf>>,
    method:Method,
}
impl<'buf> Request<'buf> {
    pub fn path(&self)->&str{
        &self.path
    }
    pub fn method(&self)->&Method{
        &self.method
    }
    pub fn query_string(&self)->Option<&QueryString>{
        self.query_string.as_ref()
    }
}
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error=parse_error;
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>{
        // match str::from_utf8(buf){
        //     Ok()->{},
        //     Err(_)=>return Err(parse_error::InvalidEncoding)
        // }
        let request=str::from_utf8(buf)?;
        // match get_next_word(request){
        //     Some((method,request))=>{},
        //     None=>parse_error::InvalidRequest
        // }
        let (method,request)=get_next_word(request).ok_or(parse_error::InvalidRequest)?;
        let (mut path,request)=get_next_word(request).ok_or(parse_error::InvalidRequest)?;
        let (protocol,_)=get_next_word(request).ok_or(parse_error::InvalidRequest)?;
        if protocol!="HTTP/1.1" {
            return Err(parse_error::InvalidProtocol);
        }
        let method:Method=method.parse()?;
        let mut query_string=None;
        if let Some(i)=path.find('?'){
            query_string=Some(QueryString::from(&path[i + 1..]));
            path=&path[..i];
        }
        Ok(Self { path, query_string, method })
    }


}
pub enum parse_error{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl parse_error{
    fn message(&self)->&str{
        match self{
        Self::InvalidRequest=>"Invalid Request",
        Self::InvalidEncoding=>"Invalid Encoding",
        Self::InvalidProtocol=>"Invalid Protocol",
        Self::InvalidMethod=>"Invalid Method",
        }
    }
}
impl From<method_error> for parse_error{
    fn from(_: method_error) -> Self{
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for parse_error{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}
fn get_next_word(request:&str)->Option<(&str,&str)>{
    for(i,char) in request.chars().enumerate(){
        if char==' ' || char=='\r' { return Some((&request[..i],&request[i+1..]))};
    }
    None
}
impl Display for parse_error{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }
}
impl Debug for parse_error{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }
}
impl Error for parse_error{

}