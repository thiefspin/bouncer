use once_cell::sync::Lazy;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};

pub static DATABASE_CONNECTION_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("db_connection_acquire", "Count of db connections acquired"), &["db_connect_acquired"])
        .expect("Could not create DATABASE_CONNECTION_COUNTER")
});

pub struct Metrics;

impl Metrics {
    pub fn init() -> PrometheusMetrics {
        let prometheus = PrometheusMetrics::new();
        prometheus
            .registry()
            .register(Box::new(DATABASE_CONNECTION_COUNTER.clone()))
            .unwrap();
        return prometheus;
    }
}