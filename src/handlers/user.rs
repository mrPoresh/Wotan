use actix_web::HttpResponse;
use actix_web::web::{Json, Data};
use sqlx::PgPool;
use color_eyre::Result;
use tracing::{info, instrument, debug};

use crate::configuration::CryptoService;
use crate::db_manager::UserService;
use crate::models::user::{NewUser, User};

use super::{AppResponse, AppError};


#[instrument(skip(user, pool, crypto))]
pub async fn create_user(
    user: Json<NewUser>, 
    pool: Data<PgPool>, 
    crypto: Data<CryptoService>
) -> AppResponse {

    info!("*** Starting create user ***");

    let menager = UserService::new(pool.clone());

    let result: Result<User> = menager.create_user(user.0, crypto.as_ref()).await;

    match result {

        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        //TOO:DO
        Err(error) => {

            let pg_error = error
                .root_cause()
                .downcast_ref::<sqlx::postgres::PgDatabaseError>()
                .ok_or_else(|| AppError::INTERNAL_ERROR.default())?;

            let error = match (pg_error.code(), pg_error.column()) {

                _ => {

                    debug!("Error creating user. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into() 
                    
                }

            };

            Err(error)

        }

    }
    
}