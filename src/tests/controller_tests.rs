// pub mod controller_tests {
use rocket::http::Status;
use rocket::local::asynchronous::Client;

use crate::create_server;
use crate::tests::testing_postgres::with_postgres_test_container;

#[rocket::async_test]
async fn test_health() {
    with_postgres_test_container(|pg_port| async move {
        let rocket = create_server(pg_port);
        let client = Client::tracked(rocket).await.unwrap();
        let req = client.get("/api/health");

        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);

        let (s1, s2) = (r1.into_string().await, r2.into_string().await);
        assert_eq!(s1, s2);
        assert_eq!(s1.unwrap(), "Service responding");
    }).await;
}
//
// #[rocket::async_test]
// async fn test_sys_info() {
//     let rocket = create_server();
//     let client = Client::tracked(rocket).await.unwrap();
//     let req = client.get("/api/sysinfo");
//
//     let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
//     assert_eq!(r1.status(), r2.status());
//     assert_eq!(r1.status(), Status::Ok);
//     assert_eq!(r2.status(), Status::Ok);
// }

#[rocket::async_test]
async fn test_container() {
    // let postgres = PostgresContainer::create_test_container();
    // println!("{}", postgres.port);
    // let rocket = create_server(pg_port);
    // let client = Client::tracked(rocket).await.unwrap();
}
// }