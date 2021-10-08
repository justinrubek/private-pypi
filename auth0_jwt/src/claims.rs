use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub scope: String,
    pub permissions: Vec<String>,
    pub aud: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        let expiration = self.expires_at();
        let now = chrono::offset::Utc::now();
        now > expiration
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.exp as i64, 0), Utc)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p.eq(permission))
    }

    pub fn has_audience(&self, audience: &str) -> bool {
        self.aud.iter().any(|aud| aud.eq(audience))
    }

    pub fn user_id(&self) -> &str {
        &self.sub
    }
}
