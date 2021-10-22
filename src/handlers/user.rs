use actix_web::HttpResponse;
use actix_web::web::{Json, Data};
use sqlx::PgPool;

use crate::db_manager::UserService;
use crate::models::user::NewUser;

pub async fn create_user(user: Json<NewUser>, pool: Data<PgPool>) -> HttpResponse {

    let menager = UserService::new(pool.clone());

    menager.create_user(user.0).await.unwrap();

    HttpResponse::Ok().finish()
}