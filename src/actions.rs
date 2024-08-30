use diesel::prelude::*;
use uuid::Uuid;

use crate::models;
type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find user by uid and return it.
pub fn find_user_by_uid(
    conn: &mut PgConnection,
    uid: i32,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::user::dsl::*;

    let users = user
        .filter(id.eq(uid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(users)
}
pub fn all(conn: &mut PgConnection) -> Result<Option<Vec<models::User>>, DbError> {
    use crate::schema::user::dsl::*;
    let users_list = user
        .load::<models::User>(conn).optional()?;
    Ok(users_list)
}
/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    conn: &mut PgConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::User, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::user::dsl::*;

    let new_user = models::User {
        id: uuid::Uuid::new_v4().as_u128() as i32, 
        name: nm.to_owned(),
    };

    diesel::insert_into(user).values(&new_user).execute(conn)?;

    Ok(new_user)
}
