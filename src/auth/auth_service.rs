use rocket_db_pools::Connection;

use crate::auth::login_error::LoginError;
use crate::auth::login_form::LoginForm;
use crate::auth::login_response::LoginResponse;
use crate::Db;
use crate::users::user_service;
use rocket_jwt::jwt;

static SECRET_KEY: &str = "very_secret_key";
#[jwt(SECRET_KEY, exp = 100)]
pub struct UserClaim {
    pub id: String,
}

pub async fn login(db: Connection<Db>, login_form: LoginForm) -> Result<LoginResponse, LoginError> {
    match user_service::get_by_email(login_form.email, db).await {
        Some(user) => {
            if (login_form.password == user.password) {
                let response = LoginResponse {
                    token: create_token()
                };
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

fn create_token() -> String {
    let user_claim = UserClaim {
        id: format!("hello_rocket_jwt"),
    };
    let token = UserClaim::sign(user_claim);
    println!("{:#?}", UserClaim::decode(token.clone()));
    token
}