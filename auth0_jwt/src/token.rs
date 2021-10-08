use jwks_client::error::Error as JwksError;
use jwks_client::jwt::Jwt;
use jwks_client::keyset::KeyStore;

use super::claims::Claims;
use super::error::TokenError;

fn auth0_keys_url(domain: &str) -> String {
    format!("https://{}/.well-known/jwks.json", domain)
}

async fn get_jwt(domain: &str, token: &str) -> Result<Jwt, JwksError> {
    let url = auth0_keys_url(domain);
    let key_store = KeyStore::new_from(url).await.unwrap();
    key_store.verify(token)
}

async fn extract_jwt(domain: &str, token: &str) -> Result<Claims, TokenError> {
    match get_jwt(domain, token).await {
        Ok(jwt) => {
            match jwt.payload().into() {
                Ok(claims) => Ok(claims),
                Err(error) => Err(TokenError::JwksError(error))
            }
        }
        Err(jwks_error) => {
           Err(TokenError::JwksError(jwks_error))
        }
    }
}

pub async fn validate_jwt(domain: &str, token: &str) -> Result<Claims, TokenError> {
    let claims = extract_jwt(domain, token).await?;

    if claims.is_expired() {
        return Err(TokenError::TokenExpired);
    }

    Ok(claims)
}
