use actix_web::HttpResponse;
use sqlx::PgPool;
use tracing::{instrument, info, debug};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::web::Data;

use crate::configuration::CryptoService;
use crate::configuration::Auth;
use crate::db_manager::UserService;

use super::AppResponse;
use super::AppError;


#[instrument]
pub async fn auth(
    basic: BasicAuth,
    pool: Data<PgPool>,
    crypto: Data<CryptoService>
) -> AppResponse {

    let email = basic.user_id();
    let password = basic
        .password()
        .unwrap();

    let menager = UserService::new(pool.clone());

    let user = menager.find_by_email(email)
        .await
        .unwrap();

    let valid = crypto
        .verify_password(password, &user.password_hash)
        .await?;

    if valid {

        let token = crypto.generate_jwt(&user.id.to_string(), &email.to_owned()).await?;
        info!("--- Token is -> {} ---", token);
        Ok(HttpResponse::Ok().json(Auth {token}))

    } else {

        debug!("Invalid password.");
        Err(AppError::INVALID_CREDENTIALS.into())

    }

}