use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::workflows;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Workflow {
    pub id: String,
    pub slug: String,
    pub secret: String,
    pub content: String,
}
