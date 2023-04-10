use crate::http::Method;

use super::server::Handler;
use super::http::{Request,Response,StatusCode};
use std::fs;
pub struct WebsiteHandler{
    public_path:String,
}
impl WebsiteHandler{
    pub fn new(public_path:String)->Self{
        Self{public_path}
    }
    pub fn read_file(&self,file_path:&str)->Option<String>{
        let path=format!("{}/{}",self.public_path,file_path);
        match fs::canonicalize(path){
            Ok(path)=>{
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                }else{
                    println!("Some one tried Directory traversal Attack!!!!!\n The path traversed is {}",file_path);
                    None
                }
            },
            Err(_)=>None,
        } 
    }
}
impl Handler for WebsiteHandler{
    fn handle_request(&mut self,request:&Request)->Response {
        match request.method(){
            Method::GET=>match request.path(){
                "/"=>Response::new(StatusCode::Ok,self.read_file("index.html")),
                "/hello"=>Response::new(StatusCode::Ok,Some("<h1>hellow there!!!</h1>".to_string())),
                path=> match self.read_file(path){
                    Some(contents)=>Response::new(StatusCode::Ok, Some(contents)),
                    None=>Response::new(StatusCode::NotFound,None),
                }
            },
            _=> Response::new(StatusCode::NotFound,None),
        }
    }
}