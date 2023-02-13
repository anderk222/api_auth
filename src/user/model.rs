use serde::{Serialize, Serializer, ser::SerializeStruct, Deserialize};
use actix_web::{Responder, body::BoxBody,http::header::ContentType,HttpRequest, HttpResponse};


#[derive(Debug,Deserialize)]
pub struct User {
   pub user_id : usize,
   pub name : String,    
   pub username : String,
   pub password : String,
   pub looked : bool,
   pub expired :bool,
   pub enabled : bool
}

impl Serialize for User{
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
           
           let mut state = serializer.serialize_struct("USER", 7)?;
           state.serialize_field("user_id", &self.user_id)?;
           state.serialize_field("name", &self.name)?;
           state.serialize_field("username", &self.username)?;
           state.serialize_field("password", &self.password)?;
           state.serialize_field("looked", &self.looked)?;
           state.serialize_field("expired", &self.expired)?;
           state.serialize_field("enabled", &self.enabled)?;
           state.end()
}
}

#[derive(Serialize)]
pub struct ResponseUser{
   message : String,
   data : User
}

impl Responder for ResponseUser{

    type Body = BoxBody;

  fn respond_to(self, _req : &HttpRequest) -> HttpResponse<Self::Body>{
    let body = serde_json::to_string(&self).unwrap();

    HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(body)
  }

}
#[derive(Serialize)]
pub struct ResponseUsers{

   pub page : u32,
   pub capacity :u8,
   pub total : usize,
   pub total_pages : u32,
   pub users : [User; 2]

}

impl Responder for ResponseUsers {

   type Body = BoxBody;

   fn respond_to(self, _req : &HttpRequest)-> HttpResponse<Self::Body>{

      let body = serde_json::to_string(&self).unwrap();

      HttpResponse::Ok().body(body)


   }

}