use std::future::Future;

use testcontainers::{clients, Container, core::WaitFor, GenericImage};
use testcontainers::clients::Cli;

const NAME: &str = "postgres";
const TAG: &str = "11-alpine";

pub async fn with_postgres_test_container<F>(test: impl Fn(u16) -> F) -> ()
    where F: Future<Output=()> {
    println!("Starting Postgres");
    let docker: Cli = clients::Cli::default();

    let postgres_image = GenericImage::new(NAME, TAG)
        .with_exposed_port(5432)
        .with_wait_for(
            WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            )
        ).with_env_var("POSTGRES_DB".to_owned(), "test".to_owned())
        .with_env_var("POSTGRES_HOST_AUTH_METHOD".to_owned(), "trust".to_owned())
        .with_env_var("POSTGRES_USER".to_owned(), "service".to_owned());

    let pg_container: &Container<GenericImage> = &docker.run(postgres_image);

    pg_container.start();

    WaitFor::seconds(60);

    // Get the PostgreSQL port
    let pg_port = pg_container.get_host_port_ipv4(5432);

    println!("Started Postgres on port: {}", pg_port);
    test(pg_port).await;
    pg_container.stop();
}
