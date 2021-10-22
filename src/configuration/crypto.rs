use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct CryptoService {
    
    pub key: Arc<String>

}

impl CryptoService {

    pub fn hash_password() {}

    pub fn verify_password() {}

    //pub async generete_token() {}

    //pub async verify_token() {}

}