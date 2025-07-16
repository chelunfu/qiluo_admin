use crate::common::snowflakeid::SnowflakeIdGenerator;
use lazy_static::lazy_static;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex, OnceCell};

use crate::config::APPCOFIG;
static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn db_conn() -> DatabaseConnection {
    let config = APPCOFIG.database.clone();
    let mut opt = ConnectOptions::new(&config.uri);
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_millis(config.connect_timeout))
        .idle_timeout(Duration::from_millis(config.idle_timeout))
        .sqlx_logging(config.enable_logging);
    let db = Database::connect(opt).await.expect("Database connected failed");
    tracing::info!("Database connected successfully");
    db
}

pub async fn db() -> &'static DatabaseConnection {
    DATABASE.get_or_init(db_conn).await
}

lazy_static! {
    static ref ID_GENERATOR_GENERATOR: Arc<Mutex<SnowflakeIdGenerator>> = {
        let config = &APPCOFIG.snowgenera;
        Arc::new(Mutex::new(SnowflakeIdGenerator::new(
            config.machine_id,
            config.node_id,
        )))
    };
}

pub async fn generator_id() -> i64 {
    let id_generator = ID_GENERATOR_GENERATOR.lock().await;
    id_generator.real_time_generate()
}

 