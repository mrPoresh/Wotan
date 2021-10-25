use std::sync::Arc;
use argonautica::{Hasher};
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;


#[derive(Debug, Clone)]
pub struct CryptoService {
    
    pub key: Arc<String>

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

    pub fn verify_password() {}

    //pub async generete_token() {}

    //pub async verify_token() {}

}