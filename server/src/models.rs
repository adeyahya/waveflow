use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
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

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "workflows"]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub secret: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Workflow {
    fn default() -> Self {
        Workflow {
            id: Uuid::new_v4().to_string(),
            name: String::from("name"),
            slug: String::from("slug"),
            secret: String::from("secret"),
            content: String::from("content"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "workflows_history"]
pub struct WorkflowHistory {
    pub id: String,
    pub workflow_id: String,
    pub content: Option<String>,
    pub is_success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "configs"]
pub struct Config {
    pub id: Option<i32>,
    pub name: String,
    pub value: String,
}
