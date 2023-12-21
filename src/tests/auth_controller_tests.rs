use rocket::form::validate::Contains;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json;

use crate::auth::login_form::LoginForm;
use crate::create_server;
use crate::tests::testing_postgres::with_postgres_test_container;

#[rocket::async_test]
async fn test_login_failure() {
    with_postgres_test_container(|pg_port| async move {
        //Given
        let rocket = create_server(pg_port).await;
        let client = Client::tracked(rocket).await.unwrap();
        let body = LoginForm{
            email: "test@mail.com".to_owned(),
            password: "password".to_owned()
        };
        let expected_response_body = r##"{"message":"User for email not found"}"##;
        let mut req = client.post("/api/auth/login");
        req.set_body(json::to_string(&body).unwrap());
        req.add_header(ContentType::JSON);

        //When
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());

        //Then
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Unauthorized);

        let (s1, s2) = (r1.into_string().await, r2.into_string().await);
        assert_eq!(s1, s2);
        assert_eq!(s1.unwrap(), expected_response_body);
    }).await;
}

#[rocket::async_test]
async fn test_login_success() {
    with_postgres_test_container(|pg_port| async move {
        //Given
        let rocket = create_server(pg_port).await;
        let client = Client::tracked(rocket).await.unwrap();
        let body = LoginForm{
            email: "setupuser@mail.com".to_owned(),
            password: "password".to_owned()
        };
        let mut req = client.post("/api/auth/login");
        req.set_body(json::to_string(&body).unwrap());
        req.add_header(ContentType::JSON);

        //When
        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());

        //Then
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);
        assert!(r1.into_string().await.contains("token"));
    }).await;
}