use actix_web::{guard::Header, web::block};
use argonautica::{Hasher, Verifier};
use chrono::{Duration, Utc};
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct CryptoService {
    
    pub key: Arc<String>,
    pub token_key: Arc<String>

}

#[derive(Serialize)]
pub struct Claims {

    //pub sub: Uuid,
    pub exp: i64,

}

#[derive(Serialize)]
pub struct Auth {

    pub token: String,

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

    pub async fn generate_jwt(&self, user_id: Uuid)  { //-> Result<String>

        //let jwt_key = self.token_key.clone();
        
        //block(move || {

            //let headers = Header::default();
            //let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            //let now = Utc::now() + Duration::hours(6);
            //let claims = Claims { sub: user_id, exp: now.timestamp() };
            //encode(&headers, &claims, &encoding_key)
            
        //})
        //.await
        //.map_err(|err| eyre!("Creating jwt token: {}", err))

    }

    pub async fn verify_token() {}

}