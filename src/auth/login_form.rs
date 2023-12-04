use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm {
    pub email: String,
    pub password: String
}