use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json;
use crate::application::database::DatabaseConfig;

use crate::auth::models::{LoginForm, LoginResponse};
use crate::create_server;
use crate::tests::testing_postgres::with_postgres_test_container;
use crate::users::models::User;

#[rocket::async_test]
async fn test_list_users() {
    with_postgres_test_container(|config| async move {
        //Given
        let rocket = create_server(config.clone()).await;
        let client = Client::tracked(rocket).await.unwrap();
        let mut req = client.get("/api/users");
        let auth_token = login_token(config.clone()).await;
        req.add_header(Header::new("Authorization", format!("Bearer {}", auth_token)));

        //When
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());

        //Then
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);
        let res = r1.into_string().await.unwrap();
        let users = json::from_str::<Vec<User>>(res.as_str()).unwrap();
        assert_eq!(users.len(), 1);
    }).await;
}

async fn login_token(config: DatabaseConfig) -> String {
    let rocket = create_server(config).await;
    let client = Client::tracked(rocket).await.unwrap();
    let body = LoginForm {
        email: "setupuser@mail.com".to_owned(),
        password: "password".to_owned(),
    };
    let mut req = client.post("/api/auth/login");
    req.set_body(json::to_string(&body).unwrap());
    req.add_header(ContentType::JSON);
    let res = req.clone().dispatch().await.into_string().await;
    json::from_str::<LoginResponse>(res.unwrap().as_str()).unwrap().token
}