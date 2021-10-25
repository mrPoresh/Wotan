use actix_web::HttpResponse;
use actix_web::web::{Json, Data};
use sqlx::PgPool;

use crate::configuration::CryptoService;
use crate::db_manager::UserService;
use crate::models::user::NewUser;

pub async fn create_user(
    user: Json<NewUser>, 
    pool: Data<PgPool>, 
    crytpo: Data<CryptoService>
) -> HttpResponse {

    let menager = UserService::new(pool.clone());

    menager.create_user(user.0, crytpo.as_ref()).await.unwrap();

    HttpResponse::Ok().finish()
    
}