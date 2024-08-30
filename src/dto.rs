use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct User{
   pub  name: String,
   pub age: i32,
}


#[derive(Deserialize,Debug)]
pub struct Info {
   pub  username: String,
}

