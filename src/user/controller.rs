use actix_web::{Responder, HttpResponse, web, Result};
use serde::{ Serialize, Deserialize };

use super::model::User;

pub async fn get_users()-> impl Responder {
    let users : [User;2] = [
        User{
            user_id : 2,
            name : "Anderson Macias",
            username : "anderk222",
            password : "ander123",
            looked : true,
            expired :false,
            enabled : true
        },
            User{
            user_id : 2,
            name : "Anderson Macias",
            username : "anderk222",
            password : "ander123",
            looked : true,
            expired :false,
            enabled : true
        }
    ];
 
    HttpResponse::Ok().body(format!("{:?}", users))
    
}

pub async fn create_user<'a>(path : web::Path<(String, String)>)->impl Responder{
    
    format!("{}",path.0)
    
}

pub async fn find_user(path : web::Path<Params>)-> Result<String>{
     

   Ok(format!(
       "{}", path.user_id
   ))
   
    
}

pub async fn update_user(req_body : User )-> User{
    
    req_body
    
}

pub async fn delete_user()-> &'static str{
    
    "deleted"
    
}
#[derive(Serialize,Deserialize)]
pub struct Params {
    user_id : usize
}