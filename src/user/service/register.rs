use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::user::model::{UserData, InsertableUser, User, SlimUser};

use actix_web::web;
use diesel::prelude::*;


pub fn register(user_data: UserData, pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
    // Connect? to our Database
    let connection = &db_connection(&pool)?;
    create_user(user_data, &connection)
}

// Adding user in the database
pub fn create_user(user_data: UserData, connection: &PgConnection) -> ServiceResult<SlimUser> {
    // users schema
    use crate::schema::users::dsl::users;

    // User data handling(in src/user/models)
    //
    // * Making salt for hashing password
    // * Generate user uuid
    // * Adding creating time and user role
    //
    let user: InsertableUser = user_data.into();
    // Adding full user in database and returning slim User as response
    let inserted_user: User = diesel::insert_into(users).values(&user).get_result(connection)?;

    Ok(inserted_user.into())

}