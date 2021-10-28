//use actix_web::{guard::Header, web::block};
use argonautica::{Hasher, Verifier};
use chrono::{Duration, Local};
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
//use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct CryptoService {
    
    pub key: Arc<String>,
    pub token_key: Arc<String>

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {

    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub email: String,

}

#[derive(Serialize)]
pub struct Auth {

    pub token: String,

}

impl Claims {

    fn with_data(user_id: &str, email: &str) -> Self {

        Claims {

            iss: "localhost".into(),
            sub: user_id.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(6)).timestamp(),
            email: email.to_owned(),

        }

    }

}

impl CryptoService {

    pub async fn hash_password(&self, password: String) -> Result<String> {

        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Hashing error: {}", err))
    }

    pub async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool> {

        Verifier::default()
            .with_secret_key(&*self.key)
            .with_hash(password_hash)
            .with_password(password)
            .verify_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Verifying error: {}", err))

    }

    pub async fn generate_jwt(&self ,user_id: &str, email: &str) -> Result<String> { //-> Result<String>

        let jwt_key = self.token_key.clone();

        let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());

        let claims = Claims::with_data(user_id, email);      //.as_str()
        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|err| eyre!("Creating jwt token: {}", err))

    }

    pub async fn verify_token(&self, token: &str) -> Result<TokenData<Claims>> {

        let jwt_key = self.token_key.clone();

        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());

        decode::<Claims>(token, &decoding_key, &Validation::default())
            .map_err(|err| eyre!("Verifying jwt token: {}", err))

    }

}