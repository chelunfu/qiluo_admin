pub mod appconfig;
use once_cell::sync::Lazy;
use appconfig::AppConfig;
pub static APPCOFIG: Lazy<AppConfig> = Lazy::new(self::AppConfig::init);