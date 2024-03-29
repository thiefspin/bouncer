use lazy_static::lazy_static;
use rocket::State;
use rocket_jwt::jwt;
use std::env;
use std::string::ToString;

use crate::{AppContext};
use crate::auth::models::{AuthToken, LoginError, LoginForm, LoginResponse};
use crate::users::models::User;
use crate::users::user_service;

lazy_static! {
    static ref SECRET_KEY: String = env::var("JWT_SECRET")
    .unwrap_or("jCDVDI7HiyPzJFKpsWfkHMlCLtD6BTGS".to_string());
}

#[jwt(SECRET_KEY, exp = 3600)]
pub struct UserClaim {
    pub user: User,
}

pub async fn login(ctx: &State<AppContext>, login_form: LoginForm) -> Result<LoginResponse, LoginError> {
    match user_service::get_by_email(login_form.email, ctx).await {
        Some(user) => {
            let user_id = user.id;
            if login_form.password == user.password {
                let response = LoginResponse {
                    token: create_token(user)
                };
                user_service::update_last_login(user_id, ctx).await;
                Ok(response)
            } else {
                let err = LoginError {
                    message: "Incorrect password".to_string()
                };
                Err(err)
            }
        }
        None => {
            let err = LoginError {
                message: "User for email not found".to_string()
            };
            Err(err)
        }
    }
}

pub async fn validate(token: AuthToken) -> bool {
    UserClaim::decode(token.clone().token).is_ok()
}

fn create_token(user: User) -> String {
    let user_claim = UserClaim { user };
    let token = UserClaim::sign(user_claim);
    println!("{:#?}", UserClaim::decode(token.clone()));
    token
}