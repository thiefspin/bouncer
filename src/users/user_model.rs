use rocket::serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;
use rocket_db_pools::sqlx::FromRow;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug, FromRow)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub phone: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct UserCreateRequest {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub phone: String,
    pub password: String
}