pub mod controller_tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    use crate::create_server;

    #[rocket::async_test]
    async fn test_health() {
        let rocket = create_server();
        let client = Client::tracked(rocket).await.unwrap();
        let req = client.get("/api/health");

        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);

        let (s1, s2) = (r1.into_string().await, r2.into_string().await);
        assert_eq!(s1, s2);
        assert_eq!(s1.unwrap(), "Service responding");
    }
}