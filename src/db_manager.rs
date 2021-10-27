use sqlx::PgPool;
//use sqlx::postgres::PgQueryAs;
use sqlx::{Postgres, Transaction};
use actix_web::{HttpResponse, web};
use uuid::Uuid;
use color_eyre::Result;

use crate::configuration::CryptoService;
use crate::models::user::{NewUser, User};


pub struct UserService {

    pool: web::Data<PgPool>

}

impl UserService {

    pub fn new(pool: web::Data<PgPool>) -> Self {

        Self { pool }

    }

    pub async fn create_user(
        &self, 
        new_user: NewUser,
        hashing: &CryptoService,
    ) {

        let user_id = Uuid::new_v4();

        //let user = sqlx::query_as...


    }

//        let hash_password = hashing.hash_password(new_user.password.clone()).await?;

//        let mut transaction = self.pool
//            .begin()
//            .await
//            .map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

//        let user_id = insert_user(&mut transaction, &new_user, hash_password)
//            .await
//            .map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

        //let token = hashing.generate_jwt(user_id).await?;

//        transaction
//            .commit()
//            .await
//            .map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

        //log::info!("-- User Uuid --> {}", user_id);

//        Ok(user_id)

//    }
    
//}

//pub async fn insert_user(
//    transaction: &mut Transaction<'_, Postgres>,
//    new_user: &NewUser,
//    password_hash: String,
//) -> Result<Uuid, sqlx::Error> {

//   let user_id = Uuid::new_v4();

//    sqlx::query!(
//        r#"
//    INSERT INTO users (id, username, email, password_hash)
//    VALUES ($1, $2, $3, $4)
//            "#,
//        user_id,
//        new_user.username,
//        new_user.email,
//        password_hash,
//    )
//    .execute(transaction)
//    .await
//    .map_err(|e| {
//        tracing::error!("Failed to execute query: {:?}", e);
//        e
//    })?;

//    Ok(user_id)

}