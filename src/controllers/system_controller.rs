use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sysinfo::SystemExt;

use crate::app::system_information;
use crate::app::system_information::SystemInformation;

#[openapi(tag = "Health")]
#[get("/health")]
pub fn health() -> &'static str {
    "Service responding"
}

#[openapi(tag = "Health")]
#[get("/sysinfo")]
pub fn system_info() -> Json<SystemInformation> {
    return Json(system_information::get_system_info())
}