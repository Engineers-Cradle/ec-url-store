use jwtk;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub keys: Vec<jwtk::jwk::Jwk>,
}

pub async fn verify_jwt(token: &str) -> bool {
    let jwks_url = crate::env_config().m2m_auth_registry_base_url + "/jwks.json";
    let keys = reqwest::get(&jwks_url).await.unwrap();


    let keys: Vec<jwtk::jwk::Jwk> = keys.json::<Keys>().await.unwrap().keys;

    let public_key = keys[0].to_verification_key().unwrap();

    let result = jwtk::verify::<serde_json::Map<String, serde_json::Value>>(token, &public_key);

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}