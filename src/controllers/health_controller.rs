use rocket_okapi::openapi;

#[openapi(tag = "Health")]
#[get("/health")]
pub fn health() -> &'static str {
    "Service responding"
}