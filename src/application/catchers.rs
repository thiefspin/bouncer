use rocket::{Catcher, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::JsonSchema;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
pub struct ApiError {
    pub status: i16,
    pub message: String,
}

pub struct Catchers;

impl Catchers {
    pub fn catchers() -> Vec<Catcher> {
        return catchers![
            internal_error,
            unauthorized, not_found
        ];
    }
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Well... This is embarrassing..."
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ApiError> {
    println!("{}", req.uri());
    Json(ApiError {
        status: 401,
        message: "Requires authentication".to_string(),
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ApiError> {
    Json(ApiError {
        status: 404,
        message: format!("{} resource not found", req.uri()),
    })
}