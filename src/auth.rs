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
    pub iat: usize,
    pub exp: usize,
}

pub fn buat_token(user_id: i64, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let sekarang = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let claims = Claims {
        sub: user_id.to_string(),
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
    validation.set_required_spec_claims(&["exp", "sub", "iat"]);

    let klaim = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)?;

    if klaim.sub.parse::<i64>().is_err() {
        return Err(Error::from(ErrorKind::InvalidToken));
    }

    Ok(klaim)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KUNCI_UJI: &str = "kunci-pengujian-jwt-yang-tidak-rahasia";

    #[test]
    fn token_valid_dapat_divalidasi() {
        let token = buat_token(42, KUNCI_UJI).unwrap();

        let claims = validasi_token(&token, KUNCI_UJI).unwrap();
        let sekarang = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(claims.sub, "42");
        assert!((sekarang..=sekarang + 1).contains(&(claims.iat as u64)));
        assert_eq!(claims.exp as u64, claims.iat as u64 + MASA_BERLAKU_DETIK);
    }

    #[test]
    fn iat_hanya_informasi_dan_tidak_menolak_token() {
        let claims = Claims {
            sub: "42".to_owned(),
            iat: usize::MAX,
            exp: (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 60) as usize,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(KUNCI_UJI.as_bytes()),
        )
        .unwrap();

        assert!(validasi_token(&token, KUNCI_UJI).is_ok());
    }

    #[test]
    fn signature_dengan_kunci_yang_salah_ditolak() {
        let token = buat_token(42, KUNCI_UJI).unwrap();

        assert!(validasi_token(&token, "kunci-yang-berbeda-dari-kunci-asli").is_err());
    }

    #[test]
    fn algoritma_selain_hs256_ditolak() {
        let klaim = serde_json::json!({
            "sub": "42",
            "exp": usize::MAX,
            "iat": 1
        });
        let token = encode(
            &Header::new(Algorithm::HS512),
            &klaim,
            &EncodingKey::from_secret(KUNCI_UJI.as_bytes()),
        )
        .unwrap();

        assert!(validasi_token(&token, KUNCI_UJI).is_err());
    }

    #[test]
    fn token_kedaluwarsa_ditolak() {
        let claims = Claims {
            sub: "42".to_owned(),
            iat: 0,
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
