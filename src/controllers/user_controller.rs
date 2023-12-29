use rocket::futures::FutureExt;
use rocket::response::status::Conflict;
use rocket::serde::json::Json;
use rocket::{Route, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::application::context::AppContext;
use crate::application::catchers::ApiError;
use crate::auth::auth_service::UserClaim;
use crate::users::models::{User, UserCreateRequest};
use crate::users::user_service;
use crate::utils::controller_utils::BaseController;

pub struct UserController;

impl BaseController for UserController {
    fn routes() -> (Vec<Route>, OpenApi) {
        return openapi_get_routes_spec![
            list_users,
            get_user,
            create
        ];
    }
}

#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(ctx: &State<AppContext>, user: UserClaim) -> Json<Vec<User>> {
    println!("{} made an API call", user.user.name);
    Json(user_service::list_users(ctx).await)
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(ctx: &State<AppContext>, id: i64) -> Option<Json<User>> {
    return user_service::get(id, ctx).map(|u_opt| u_opt.map(|u| Json(u))).await;
}

#[openapi(tag = "Users")]
#[post("/", format = "application/json", data = "<user>")]
pub async fn create(user: Json<UserCreateRequest>, ctx: &State<AppContext>) -> Result<Json<User>, Conflict<Json<ApiError>>> {
    let inner = &user.into_inner();
    return match user_service::create(inner, ctx).await {
        Some(result) => Ok(Json(result)),
        None => Err(Conflict(Some(Json(ApiError {
            status: 409,
            message: format!("User with email {} already exists", inner.email),
        }))))
    };
}