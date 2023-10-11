#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, Route};
use rocket_db_pools::{Database, sqlx};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec, rapidoc::*, swagger_ui::*};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::{OpenApiSettings, UrlObject};

#[path = "./users/mod.rs"]
mod users;
#[path = "application/mod.rs"]
mod app;

#[path = "./controllers/user_controller.rs"]
mod user_controller;

#[cfg(test)]
mod tests;

#[derive(Database)]
#[database("test")]
pub struct Users(sqlx::PgPool);

fn get_user_controller_routes() -> (Vec<Route>, OpenApi) {
    return openapi_get_routes_spec![
        user_controller::list_users,
        user_controller::get_user
    ];
}

#[openapi(tag = "Health")]
#[get("/health")]
fn health() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    // let db_pool = app::database::init().await;
    // sqlx::migrate!().run(&db_pool).await.expect("Failed to run migrations");
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

fn create_server() -> Rocket<Build> {
    let mut build_rocket = rocket::build()
        .attach(Users::init())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../api/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
    build_rocket, "/api".to_owned(), settings,
        "/" => openapi_get_routes_spec![health],
        "/users" => get_user_controller_routes(),
    }
    build_rocket
}