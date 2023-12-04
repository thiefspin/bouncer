use rocket::Request;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::JsonSchema;
use sqlx::FromRow;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
pub struct ApiError {
    status: i16,
    message: String,
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