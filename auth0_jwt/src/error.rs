use thiserror::Error;
use jwks_client::error::Error as JwksError;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("token expired")]
    TokenExpired,
    #[error("token used early")]
    TokenEarly,
    #[error("failed to verify token: {0}")]
    JwksError(JwksError),
}
