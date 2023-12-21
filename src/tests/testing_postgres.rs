use rocket::form::FromForm;
use testcontainers::{clients, Container, core::WaitFor, GenericImage, Image};

// use crate::{core::WaitFor, Image};
use std::collections::HashMap;
use std::future::Future;
use testcontainers::clients::Cli;

const NAME: &str = "postgres";
const TAG: &str = "11-alpine";

#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());

        Self { env_vars }
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item=(&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![55004]
    }
}

pub async fn with_postgres_test_container<F>(test: impl Fn(u16) -> F) -> ()
    where F: Future<Output = ()> {
    println!("Starting Postgres");
    let docker: Cli = clients::Cli::default();

    let mut env_vars = HashMap::new();
    env_vars.insert("POSTGRES_DB".to_owned(), "test".to_owned());
    env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());

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
    test(pg_port).await
}

// pub struct PostgresContainer<'a> {
//     pub container: &'a Container<'a, GenericImage>,
//     pub port: u16
// }
//
// impl PostgresContainer<'_> {
//     pub fn create_test_container<'a>() -> PostgresContainer<'a> {
//         println!("Starting Postgres");
//         let docker: Cli = clients::Cli::default();
//
//         // Define a PostgreSQL container image
//         // let postgres_image = Postgres::default();
//
//         let mut env_vars = HashMap::new();
//         env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
//         env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());
//
//         let postgres_image = GenericImage::new(NAME, TAG)
//             .with_exposed_port(5432)
//             .with_wait_for(
//                 WaitFor::message_on_stderr(
//                     "database system is ready to accept connections",
//                 )
//             ).with_env_var("POSTGRES_DB".to_owned(), "postgres".to_owned())
//             .with_env_var("POSTGRES_HOST_AUTH_METHOD".to_owned(), "trust".to_owned());
//
//         let pg_container: &Container<GenericImage> = &docker.run(postgres_image);
//
//         pg_container.start();
//
//         WaitFor::seconds(60);
//
//         // Get the PostgreSQL port
//         let pg_port = pg_container.get_host_port_ipv4(5432);
//
//         println!("Started Postgres on port: {}", pg_port);
//         return PostgresContainer {
//             container: pg_container.to_owned(),
//             port: pg_port.clone()
//         }
//
//         // // Define the connection to the Postgress client
//         // let (client, connection) = tokio_postgres::Config::new()
//         //     .user("postgres")
//         //     .password("postgres")
//         //     .host("localhost")
//         //     .port(pg_port)
//         //     .dbname("postgres")
//         //     .connect(tokio_postgres::NoTls)
//         //     .await
//         //     .unwrap();
//         //
//         // // Spawn connection
//         // tokio::spawn(async move {
//         //     if let Err(error) = connection.await {
//         //         println!("Connection error: {}", error);
//         //     }
//         // });
//     }
//
//     pub fn stop(&self) {
//         &self.container.stop();
//     }
// }