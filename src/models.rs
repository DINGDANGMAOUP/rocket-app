use serde::{Deserialize, Serialize};

use crate::schema::user;

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}