use actix_web::HttpResponse;
use actix_web::web::{Json, Data};
use sqlx::PgPool;
use tracing::{info, debug, instrument};

use crate::configuration::CryptoService;
use crate::db_manager::UserService;
use crate::models::user::NewUser;


#[instrument(skip(user, pool, crypto))]
pub async fn create_user(
    user: Json<NewUser>, 
    pool: Data<PgPool>, 
    crypto: Data<CryptoService>
) -> HttpResponse {

    let menager = UserService::new(pool.clone());

    //menager.create_user(user.0, crypto.as_ref()).await.unwrap();

    HttpResponse::Ok().finish()
    
}