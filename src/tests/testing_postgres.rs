use std::future::Future;

use testcontainers::{clients, Container, core::WaitFor, GenericImage};
use testcontainers::clients::Cli;
use crate::application::database::DatabaseConfig;

const NAME: &str = "postgres";
const TAG: &str = "11-alpine";
const USER: &str = "service";
const PASSWORD: &str = "password";
const DATABASE_NAME: &str = "test";
const DATABASE_PORT_INTERNAL: u16 = 5432;

pub async fn with_postgres_test_container<F>(test: impl Fn(DatabaseConfig) -> F) -> ()
    where F: Future<Output=()> {
    println!("Starting Postgres");
    let docker: Cli = clients::Cli::default();

    let postgres_image = GenericImage::new(NAME, TAG)
        .with_exposed_port(DATABASE_PORT_INTERNAL)
        .with_wait_for(
            WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            )
        ).with_env_var("POSTGRES_DB".to_owned(), DATABASE_NAME.to_owned())
        .with_env_var("POSTGRES_HOST_AUTH_METHOD".to_owned(), "trust".to_owned())
        .with_env_var("POSTGRES_USER".to_owned(), USER.to_owned());

    let pg_container: &Container<GenericImage> = &docker.run(postgres_image);

    pg_container.start();

    WaitFor::seconds(60);

    // Get the PostgreSQL port
    let pg_port = pg_container.get_host_port_ipv4(5432);

    println!("Started Postgres on port: {}", pg_port);
    let config = DatabaseConfig {
        port: pg_port,
        host: "localhost".to_string(),
        user: USER.to_string(),
        password: PASSWORD.to_string(),
        database_name: DATABASE_NAME.to_string(),
        min_connections: 1,
        max_connections: 1
    };
    test(config).await;
    pg_container.stop();
}
