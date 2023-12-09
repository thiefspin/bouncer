#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, Route};
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, sqlx};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;

use controllers::{auth_controller, health_controller, user_controller};

#[path = "./users/mod.rs"]
mod users;

#[path = "./auth/mod.rs"]
mod auth;

#[path = "application/mod.rs"]
mod app;

#[cfg(test)]
#[path = "tests/controller_tests.rs"]
mod tests;

#[path = "controllers/mod.rs"]
mod controllers;
mod utils;

#[derive(Database)]
#[database("test")]
pub struct Db(sqlx::PgPool);

fn get_user_controller_routes() -> (Vec<Route>, OpenApi) {
    return openapi_get_routes_spec![
        user_controller::list_users,
        user_controller::get_user,
        user_controller::create
    ];
}

fn get_auth_controller_routes() -> (Vec<Route>, OpenApi) {
    return openapi_get_routes_spec![
        auth_controller::login
    ];
}

#[rocket::main]
async fn main() {
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

fn create_server() -> Rocket<Build> {
    let mut build_rocket = rocket::build()
        .attach(Db::init())
        .register("/", catchers![app::catchers::internal_error, app::catchers::unauthorized])
        .attach(AdHoc::try_on_ignite("DB Migrations", app::database_migrations::run_migrations))
        .mount("/swagger-ui/", app::swagger::swagger_ui())
        .mount("/rapidoc/", app::swagger::swagger_doc_config());
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "" => openapi_get_routes_spec![health_controller::health, health_controller::system_info],
        "/users" => get_user_controller_routes(),
        "/auth" => get_auth_controller_routes()
    }
    build_rocket
}