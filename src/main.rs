#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs};
use rocket_okapi::settings::OpenApiSettings;

use crate::application::catchers::Catchers;
use crate::application::context::AppContext;
use crate::application::database::{Database, DatabaseConfig};
use crate::application::swagger::Swagger;
use crate::controllers::auth_controller::AuthController;
use crate::controllers::system_controller::SystemController;
use crate::controllers::user_controller::UserController;
use crate::utils::controller_utils::BaseController;

mod utils;
mod controllers;
mod application;
mod users;
mod auth;

#[cfg(test)]
mod tests;

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
        .mount("/swagger-ui/", Swagger::swagger_ui())
        .mount("/rapidoc/", Swagger::swagger_doc_config());
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "" => SystemController::routes(),
        "/auth" => AuthController::routes(),
        "/users" => UserController::routes()
    }
    build_rocket.manage(AppContext { database: db })
}