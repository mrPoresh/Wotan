//use sqlx::{Executor, PgPool};
//use sqlx::postgres::PgQueryAs;
//use sqlx::{Postgres, Transaction};
use sqlx::PgPool;
use actix_web::web;
use uuid::Uuid;
use color_eyre::Result;
use tracing::{info, instrument};

use crate::configuration::CryptoService;
use crate::models::user::{NewUser, User};


pub const UNIQUE_VIOLATION_CODE: &str = "23505";


#[derive(Debug)]
pub struct UserService {

    pool: web::Data<PgPool>

}

impl UserService {

    pub fn new(pool: web::Data<PgPool>) -> Self {

        Self { pool }

    }

    #[instrument(skip(self, new_user, hashing))]
    pub async fn create_user(
        &self, 
        new_user: NewUser,
        hashing: &CryptoService,
    ) -> Result<User> {
    
        let hash_password = hashing.hash_password(new_user.password.clone()).await?;

        let user_id = Uuid::new_v4();

        let pool = self.pool.as_ref();

        info!("--- Saving new user (id) -> {} ---", user_id);

        let token = hashing.generate_jwt(&user_id.to_string().clone(), &new_user.email.clone()).await?;

        info!("---> Test token <--- ---> {}", token);

        let decode_token = hashing.verify_token(&token).await?;

        info!("---> Test token decode <--- ---> {:?}", decode_token);

        let user = sqlx::query_as::<_, User>(
            "insert into users (id, username, email, password_hash) values ($1, $2, $3, $4) returning *",
        )
        .bind(user_id)
        .bind(new_user.username)
        .bind(new_user.email)
        .bind(hash_password)
        .fetch_one(pool)
        .await?;

        info!("*** Complite saving ***");

        Ok(user)

    }
    
}