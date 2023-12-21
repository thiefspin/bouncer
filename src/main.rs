#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, Route};
use rocket::figment::Figment;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{Config, Database, sqlx};
use rocket_okapi::{JsonSchema, mount_endpoints_and_merged_docs, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgPoolOptions;

use controllers::{auth_controller, system_controller, user_controller};

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
    db_pool: PgPool
}

fn get_user_controller_routes() -> (Vec<Route>, OpenApi) {
    return openapi_get_routes_spec![
        user_controller::list_users,
        user_controller::get_user,
        user_controller::create
    ];
}

fn get_auth_controller_routes() -> (Vec<Route>, OpenApi) {
    return openapi_get_routes_spec![
        auth_controller::login,
        auth_controller::validate
    ];
}

#[rocket::main]
async fn main() {
    let server = create_server(5432).await;
    let launch_result = server.launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
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
}

async fn create_server(db_port: u16) -> Rocket<Build> {
    let database_url = format!("postgres://service:password@localhost:{}/test", db_port);
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => {
            println!("✅ Migrations ran successfully");
        }
        Err(err) => {
            println!("🔥 Migrations could not run successfully");
            println!("{}", err);
        }
    }
    let db_config = create_db_config(db_port);
    let mut build_rocket = rocket::custom(db_config)
        .register("/", catchers![app::catchers::internal_error, app::catchers::unauthorized])
        .mount("/swagger-ui/", app::swagger::swagger_ui())
        .mount("/rapidoc/", app::swagger::swagger_doc_config());
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "" => openapi_get_routes_spec![system_controller::health, system_controller::system_info],
        "/users" => get_user_controller_routes(),
        "/auth" => get_auth_controller_routes()
    }
    build_rocket.manage(AppContext{db_pool: pool})

}