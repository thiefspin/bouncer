use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
#[derive(Responder)]
#[response(status = 401, content_type = "json")]
pub struct LoginError {
    pub message: String
}