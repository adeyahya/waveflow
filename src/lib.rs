#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::HttpRequest;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use hmac::{Hmac, Mac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub mod models;
pub mod schema;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WebConfig {
    pub app_secret: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Repository {
    pub html_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct _FormData {
    pub repository: Repository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpErrorMessage {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub username: String,
    pub email: String,
    pub access_token: String,
}

pub type HmacSha256 = Hmac<Sha256>;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_signature<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Hub-Signature-256")?.to_str().ok()
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
