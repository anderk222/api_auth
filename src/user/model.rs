use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug)]
pub struct User {
   pub user_id : usize,
   pub name : &'static str,    
   pub username : &'static  str,
   pub password : &'static  str,
   pub looked : bool,
   pub expired :bool,
   pub enabled : bool
}

impl Serialize for User{
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
           
           let mut state = serializer.serialize_struct("uSER", 7)?;
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