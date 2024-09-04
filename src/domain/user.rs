use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User{
    pub  id: Option<i32>,
    pub  name: Option<String>,
 }
crud!( User{},"t_user");