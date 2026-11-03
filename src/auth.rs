use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const MASA_BERLAKU_DETIK: u64 = 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn buat_token(username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let sekarang = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let claims = Claims {
        sub: username.to_owned(),
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
    validation.set_required_spec_claims(&["exp", "sub"]);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KUNCI_UJI: &str = "kunci-pengujian-jwt-yang-tidak-rahasia";

    #[test]
    fn token_valid_dapat_divalidasi() {
        let token = buat_token("pengguna-uji", KUNCI_UJI).unwrap();

        let claims = validasi_token(&token, KUNCI_UJI).unwrap();

        assert_eq!(claims.sub, "pengguna-uji");
    }

    #[test]
    fn token_kedaluwarsa_ditolak() {
        let claims = Claims {
            sub: "pengguna-uji".to_owned(),
            exp: 0,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(KUNCI_UJI.as_bytes()),
        )
        .unwrap();

        assert!(validasi_token(&token, KUNCI_UJI).is_err());
    }
}
