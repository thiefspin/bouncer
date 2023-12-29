use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm {
    pub email: String,
    pub password: String
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
#[derive(Responder)]
#[response(status = 401, content_type = "json")]
pub struct LoginError {
    pub message: String
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    pub token: String
}