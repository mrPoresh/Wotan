use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::Deserialize; //Serialize


//#[derive(Debug, Deserialize)]
#[derive(Debug, Deserialize)]
pub struct NewUser {

    pub username: String,
    pub email: String,
    pub password: String,

}

//#[derive(sqlx::FromRow, Serialize)]
#[derive(Debug, sqlx::FromRow)]
pub struct User {

    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    //#[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    //pub bio: Option<String>,
    //pub image: Option<String>,
    //#[serde(skip_serializing)]
    //pub email_verified: bool,
    //#[serde(skip_serializing)]
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

}