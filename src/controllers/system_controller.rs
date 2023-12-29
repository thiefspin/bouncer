use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;

use crate::application::system_information::SystemInformation;
use crate::utils::controller_utils::BaseController;

pub struct SystemController;

impl BaseController for SystemController {
    fn routes() -> (Vec<Route>, OpenApi) {
        return openapi_get_routes_spec![
            health,
            system_info
        ];
    }
}

#[openapi(tag = "Health")]
#[get("/health")]
pub fn health() -> &'static str {
    "Service responding"
}

#[openapi(tag = "Health")]
#[get("/sysinfo")]
pub fn system_info() -> Json<SystemInformation> {
    return Json(SystemInformation::collect())
}