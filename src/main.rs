#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, Route};
use rocket::figment::Figment;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{Config, sqlx};
use rocket_okapi::{JsonSchema, mount_endpoints_and_merged_docs, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::FromRow;

use controllers::{system_controller, user_controller};
use crate::app::catchers::Catchers;

use crate::app::database::{Database, DatabaseConfig};
use crate::controllers::auth_controller::AuthController;
use crate::controllers::system_controller::SystemController;
use crate::controllers::user_controller::UserController;
use crate::utils::controller_utils::BaseController;

#[path = "./users/mod.rs"]
mod users;

#[path = "./auth/mod.rs"]
mod auth;

#[path = "application/mod.rs"]
mod app;

#[cfg(test)]
mod tests;

#[path = "controllers/mod.rs"]
mod controllers;
mod utils;

#[derive(Clone, Debug)]
pub struct AppContext {
    database: Database
}

#[rocket::main]
async fn main() {
    let db_config = DatabaseConfig::init();
    let server = create_server(db_config).await;
    let launch_result = server.launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

async fn create_server(db_config: DatabaseConfig) -> Rocket<Build> {
    let db = Database::init(db_config).await;
    db.run_migrations().await;
    let mut build_rocket = rocket::build()
        .register("/", Catchers::catchers())
        .mount("/swagger-ui/", app::swagger::swagger_ui())
        .mount("/rapidoc/", app::swagger::swagger_doc_config());
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "" => SystemController::routes(),
        "/auth" => AuthController::routes(),
        "/users" => UserController::routes()
    }
    build_rocket.manage(AppContext{database: db})

}