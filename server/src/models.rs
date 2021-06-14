use serde::{Deserialize, Serialize};

use crate::schema::*;

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
    pub name: String,
    pub slug: String,
    pub secret: String,
    pub content: String,
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
