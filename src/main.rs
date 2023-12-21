#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, Route};
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::form::FromForm;
use rocket_db_pools::{Config, Database, sqlx};
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
mod tests;
// #[path = "tests/controller_tests.rs"]

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
    let launch_result = create_server(5432).launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

fn init_datasource(build: Rocket<Build>) -> Rocket<Build> {
    build.attach(Db::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", app::database_migrations::run_migrations))
}

fn create_db_config(port: u16) -> Figment {
    let figment = rocket::Config::figment()
        .merge(("databases.test", Config {
            url: format!("postgres://service:password@localhost:{}/test", port).into(),
            min_connections: None,
            max_connections: 10,
            connect_timeout: 3,
            idle_timeout: Some(30000),
        }));
    return figment;
    // Config {
    //     url: format!("postgres://postgres:postgres@localhost:{}/test", port),
    //     min_connections: None,
    //     max_connections: 10,
    //     connect_timeout: 5,
    //     idle_timeout: Some(30000),
    // }
}

fn create_server(db_port: u16) -> Rocket<Build> {
    let db_config = create_db_config(db_port);
    let mut build_rocket = rocket::custom(db_config)
        // .attach(AdHoc::try_on_ignite("Db Config", db_config))
        // .attach(Db::init())
        .register("/", catchers![app::catchers::internal_error, app::catchers::unauthorized])
        // .attach(AdHoc::try_on_ignite("DB Migrations", app::database_migrations::run_migrations))
        .mount("/swagger-ui/", app::swagger::swagger_ui())
        .mount("/rapidoc/", app::swagger::swagger_doc_config());
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "" => openapi_get_routes_spec![health_controller::health, health_controller::system_info],
        "/users" => get_user_controller_routes(),
        "/auth" => get_auth_controller_routes()
    }
    init_datasource(build_rocket)
}