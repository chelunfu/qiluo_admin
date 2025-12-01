use std::sync::Arc;

use lazy_static::lazy_static;

use crate::common::snowflakeid::SnowflakeIdGenerator;
use crate::config::APPCOFIG;

lazy_static! {
    static ref ID_GENERATOR_GENERATOR: Arc<tokio::sync::Mutex<SnowflakeIdGenerator>> = {
        let config = &APPCOFIG.snowgenera;
        Arc::new(tokio::sync::Mutex::new(SnowflakeIdGenerator::new(
            config.machine_id,
            config.node_id,
        )))
    };
}

pub async fn generator_id() -> i64 {
    let id_generator = ID_GENERATOR_GENERATOR.lock().await;
    id_generator.real_time_generate()
}
