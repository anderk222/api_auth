use actix_web::{Responder, HttpResponse, web};

use super::model::*;

pub async fn get_users()-> ResponseUsers {
    let users : [User;2] = [
        User{
            user_id : 2,
            name : String::from("Anderson Macias"),
            username : String::from("anderk222"),
            password : String::from("ander123"),
            looked : true,
            expired :false,
            enabled : true
        },
            User{
            user_id : 2,
            name : String::from("Anderson Macias"),
            username : String::from("anderk222"),
            password : String::from("ander123"),
            looked : true,
            expired :false,
            enabled : true
        }
    ];

    ResponseUsers{
        page : 0,
        capacity : 10,
        total : 2,
        total_pages : 1,
        users
    }
 
    
    
}

pub async fn create_user<'a>( user : web::Json<User>)->impl Responder{
    
   HttpResponse::Ok().body(format!("{:?}",user))
     
}

pub async fn find_user(path : web::Path<usize>)-> impl Responder{

   let id = path.into_inner();

   format!("{}", id)
   
}

pub async fn update_user(user : web::Json<User> )-> impl Responder{
    
 format!("{:?}", user)
    
}

pub async fn delete_user()-> &'static str{
    
    "deleted"
    
}
