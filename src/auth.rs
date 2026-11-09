use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{Error, ErrorKind},
};
use serde::{Deserialize, Serialize};

const MASA_BERLAKU_DETIK: u64 = 30 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
}

fn role_diizinkan(role: &str) -> bool {
    matches!(role, "admin" | "petugas_gudang")
}

pub fn buat_token(
    user_id: i64,
    role: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    if !role_diizinkan(role) {
        return Err(Error::from(ErrorKind::InvalidToken));
    }

    let sekarang = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_owned(),
        iat: sekarang as usize,
        exp: (sekarang + MASA_BERLAKU_DETIK) as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validasi_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0;
    validation.set_required_spec_claims(&["exp", "sub", "iat", "role"]);

    let klaim = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)?;

    if klaim.sub.parse::<i64>().is_err() || !role_diizinkan(&klaim.role) {
        return Err(Error::from(ErrorKind::InvalidToken));
    }

    Ok(klaim)
}

