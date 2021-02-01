use crate::database::Database;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone)]
pub struct State {
    pub database: Database,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Eq, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserNoId {
    pub username: String,
}
