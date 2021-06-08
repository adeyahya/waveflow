#[macro_use]
extern crate diesel;
extern crate dotenv;
use hmac::{Hmac, Mac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

type HmacSha256 = Hmac<Sha256>;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_jwt_token(key: String, sub: String) -> Result<String, jwt::Error> {
    let mac: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    let token_str = claims.sign_with_key(&mac)?;
    Ok(token_str.to_owned())
}

pub fn calculate_sha256_signature(buff: String, secret: String) -> Option<String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(buff.as_bytes());
    let result = mac.finalize().into_bytes();
    let r2 = hex::encode(&result);

    Some(r2)
}
