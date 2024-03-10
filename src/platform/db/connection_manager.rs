use deadpool_diesel::postgres::{Manager, Pool};

use crate::platform::config::Config;

pub fn setup_connection_pool(config: &Config) -> Pool {
    let manager = Manager::new(
        &config.db_url().to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );
    Pool::builder(manager).build().unwrap()
}
