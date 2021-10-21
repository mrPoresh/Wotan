use sqlx::PgPool;
use sqlx::{Postgres, Transaction};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::models::user::NewUser;


pub struct UserMenager {

    pool: web::Data<PgPool>

}

impl UserMenager {

    pub fn new(pool: web::Data<PgPool>) -> Self {

        Self { pool }

    }

    pub async fn create_user(
        &self, 
        new_user: NewUser
    ) -> Result<HttpResponse, HttpResponse> {

        let mut transaction = self.pool
            .begin()
            .await
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        let user_id = insert_user(&mut transaction, &new_user)
            .await
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        transaction
            .commit()
            .await
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        println!("-- User Uuid --> {}", user_id);

        Ok(HttpResponse::Ok().finish())

    }
    
}

pub async fn insert_user(
    transaction: &mut Transaction<'_, Postgres>,
    new_user: &NewUser,
) -> Result<Uuid, sqlx::Error> {

    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
    INSERT INTO users (id, username, email, password_hash)
    VALUES ($1, $2, $3, $4)
            "#,
        user_id,
        new_user.username,
        new_user.email,
        new_user.password
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(user_id)

}