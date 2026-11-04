use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::{SaltString, rand_core::OsRng};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{Error, ErrorKind},
};
use serde::{Deserialize, Serialize};

const MASA_BERLAKU_DETIK: u64 = 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub jti: String,
}

pub fn buat_token(user_id: i64, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let sekarang = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (sekarang + MASA_BERLAKU_DETIK) as usize,
        jti: SaltString::generate(&mut OsRng).to_string(),
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
    validation.set_required_spec_claims(&["exp", "sub", "jti"]);

    let klaim = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)?;

    if klaim.sub.parse::<i64>().is_err() || klaim.jti.trim().is_empty() {
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
        assert!(!claims.jti.is_empty());
        assert!((sekarang + 3599..=sekarang + 3600).contains(&(claims.exp as u64)));
    }

    #[test]
    fn setiap_token_memiliki_jti_berbeda() {
        let token_pertama = buat_token(42, KUNCI_UJI).unwrap();
        let token_kedua = buat_token(42, KUNCI_UJI).unwrap();

        let klaim_pertama = validasi_token(&token_pertama, KUNCI_UJI).unwrap();
        let klaim_kedua = validasi_token(&token_kedua, KUNCI_UJI).unwrap();

        assert_ne!(klaim_pertama.jti, klaim_kedua.jti);
    }

    #[test]
    fn token_tanpa_jti_ditolak() {
        let klaim = serde_json::json!({
            "sub": "42",
            "exp": usize::MAX
        });
        let token = encode(
            &Header::new(Algorithm::HS256),
            &klaim,
            &EncodingKey::from_secret(KUNCI_UJI.as_bytes()),
        )
        .unwrap();

        assert!(validasi_token(&token, KUNCI_UJI).is_err());
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
            "jti": "jti-uji"
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
            exp: 0,
            jti: "jti-kedaluwarsa".to_owned(),
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
