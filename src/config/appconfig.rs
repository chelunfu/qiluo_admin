use fs_err as fs;

use crate::common::{error::Result, tera};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_variant::to_variant_name;
use std::collections::BTreeMap;
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub logger: Logger,
    pub server: Server,
    pub database: Database,
    pub cache: CacheConfig,
    #[serde(default)]
    pub workers: Workers,
    pub auth: Auth,
    pub mailer: Option<Mailer>,
    pub snowgenera: SnowGenerator,
    pub system: System,
}

impl AppConfig {
    pub fn init() -> Self {
        let env = env::var("environment").unwrap_or_else(|_| "development".to_string());
        AppConfig::load(env).unwrap()
    }

    pub fn load(env: String) -> Result<Self> {
        let path = env::current_dir().unwrap();

        let path = path.join("config");
        let files = path.join(format!("{env}.yaml"));

        let content = fs::read_to_string(files).expect("msg");
        let rendered = tera::render_string(&content, &json!({}))?;
        let s = serde_yaml::from_str(&rendered);
        let appconfi: AppConfig = s.unwrap();
        Ok(appconfi)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct System {
    /// 超级管理员账号
    pub super_role: Vec<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ssl {
    pub enable: bool,
    pub key: String,
    pub cert: String,
}

//
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SnowGenerator {
    pub machine_id: i32,
    pub node_id: i32,
}
/// Logger configuration
///
/// The Loco logging stack is built on `tracing`, using a carefuly
/// crafted stack of filters and subscribers. We filter out noise,
/// apply a log level across your app, and sort out back traces for
/// a great developer experience.
///
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// logger:
///   enable: true
///   pretty_backtrace: true
///   level: debug
///   format: compact
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Logger {
    pub enable: bool,

    /// Enable nice display of backtraces, in development this should be on.
    /// Turn it off in performance sensitive production deployments.
    #[serde(default)]
    pub pretty_backtrace: bool,

    /// Set the logger level.
    ///
    /// * options: `trace` | `debug` | `info` | `warn` | `error`
    pub level: LogLevel,

    /// Set the logger format.
    ///
    /// * options: `compact` | `pretty` | `json`
    pub format: Format,

    /// Override our custom tracing filter.
    ///
    /// Set this to your own filter if you want to see traces from internal
    /// libraries. See more [here](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives)
    pub override_filter: Option<String>,

    pub log_file_cache: bool,
    pub log_dir: String,
    pub file_name: String,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    /// The "off" level.
    #[serde(rename = "off")]
    Off,
    /// The "trace" level.
    #[serde(rename = "trace")]
    Trace,
    /// The "debug" level.
    #[serde(rename = "debug")]
    Debug,
    /// The "info" level.
    #[serde(rename = "info")]
    #[default]
    Info,
    /// The "warn" level.
    #[serde(rename = "warn")]
    Warn,
    /// The "error" level.
    #[serde(rename = "error")]
    Error,
}
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_variant_name(self).expect("only enum supported").fmt(f)
    }
}
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub enum Format {
    #[serde(rename = "compact")]
    #[default]
    Compact,
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "json")]
    Json,
}
/// Database configuration
///
/// Configures the [SeaORM](https://www.sea-ql.org/SeaORM/) connection and pool, as well as Loco's additional DB
/// management utils such as `auto_migrate`, `truncate` and `recreate`.
///
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// database:
///   uri: {{ get_env(name="DATABASE_URL", default="...") }}
///   enable_logging: true
///   connect_timeout: 500
///   idle_timeout: 500
///   min_connections: 1
///   max_connections: 1
///   auto_migrate: true
///   dangerously_truncate: false
///   dangerously_recreate: false
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Database {
    /// The URI for connecting to the database. For example:
    /// * Postgres: `postgres://root:12341234@localhost:5432/myapp_development`
    /// * Sqlite: `sqlite://db.sqlite?mode=rwc`
    pub uri: String,

    /// Enable `SQLx` statement logging
    pub enable_logging: bool,

    /// Minimum number of connections for a pool
    pub min_connections: u32,

    /// Maximum number of connections for a pool
    pub max_connections: u32,

    /// Set the timeout duration when acquiring a connection
    pub connect_timeout: u64,

    /// Set the idle duration before closing a connection
    pub idle_timeout: u64,

    /// Set the timeout for acquiring a connection
    pub acquire_timeout: Option<u64>,

    /// Run migration up when application loads. It is recommended to turn it on
    /// in development. In production keep it off, and explicitly migrate your
    /// database every time you need.
    #[serde(default)]
    pub auto_migrate: bool,

    /// Truncate database when application loads. It will delete data from your
    /// tables. Commonly used in `test`.
    #[serde(default)]
    pub dangerously_truncate: bool,

    /// Recreate schema when application loads. Use it when you want to reset
    /// your database *and* structure (drop), this also deletes all of the data.
    /// Useful when you're just sketching out your project and trying out
    /// various things in development.
    #[serde(default)]
    pub dangerously_recreate: bool,
}



/// User authentication configuration.
///
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// auth:
///   jwt:
///     secret: <your secret>
///     expiration: 604800 # 7 days
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Auth {
    /// JWT authentication config
    pub jwt: JWT,
}

/// JWT configuration structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JWT {
    /// The location where JWT tokens are expected to be found during
    /// authentication.
    pub location: Option<JWTLocation>,
    /// The secret key For JWT token
    pub secret: String,
    /// The expiration time for authentication tokens
    pub expiration: i64,
}

/// Defines the authentication mechanism for middleware.
///
/// This enum represents various ways to authenticate using JSON Web Tokens
/// (JWT) within middleware.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "from")]
pub enum JWTLocation {
    /// Authenticate using a Bearer token.
    Bearer,
    /// Authenticate using a token passed as a query parameter.
    Query { name: String },
    /// Authenticate using a token stored in a cookie.
    Cookie { name: String },
}

/// Server configuration structure.
///
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// server:
///   port: {{ get_env(name="NODE_PORT", default=3000) }}
///   host: http://localhost
///   middlewares:
///     limit_payload:
///       enable: true
///       body_limit: 5mb
///     logger:
///       enable: true
///     catch_panic:
///       enable: true
///     timeout_request:
///       enable: true
///       timeout: 5000
///     compression:
///       enable: true
///     cors:
///       enable: true
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    /// The server domain name.
    pub domainname: String,
    /// The address on which the server should listen on for incoming
    /// connections.
    #[serde(default = "default_binding")]
    pub binding: String,
    /// The port on which the server should listen for incoming connections.
    pub port: i32,
    /// The webserver host
    pub host: String,
    /// Identify via the `Server` header
    pub ident: Option<String>,
    /// The directory where the static files will be served from.
    pub static_dir: String,
    /// The directory where the web server should serve static files from.
    pub web_dir: String,
    /// The directory where the web server should serve uploaded files from.
    pub upload_dir: String,
    /// Middleware configurations for the server, including payload limits,
    /// logging, and error handling.
    pub middlewares: Middlewares,
    /// SSL configuration for the server.
    pub ssl: Ssl,
}
const DEFAULT_SERVER_BINDING: &str = "0.0.0.0";
fn default_binding() -> String {
    DEFAULT_SERVER_BINDING.to_string()
}

impl Server {
    #[must_use]
    pub fn full_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Background worker configuration
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// workers:
///   mode: BackgroundQueue
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Workers { 
    /// Custom queue names declaration. Required if you set up a dedicated
    /// worker against a dedicated queue.
    pub queues: Option<Vec<String>>,
    /// The number of workers to start
    pub num_workers: u16
}


/// Server middleware configuration structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Middlewares {
    /// Middleware that enable compression for the response.
    pub compression: Option<EnableMiddleware>,
    /// Middleware that enable etag cache headers.
    pub etag: Option<EnableMiddleware>,
    /// Middleware that limit the payload request.
    pub limit_payload: Option<LimitPayloadMiddleware>,
    /// Middleware that improve the tracing logger and adding trace id for each
    /// request.
    pub logger: Option<EnableMiddleware>,
    /// catch any code panic and log the error.
    pub catch_panic: Option<EnableMiddleware>,
    /// Setting a global timeout for the requests
    pub timeout_request: Option<TimeoutRequestMiddleware>,
    /// Setting cors configuration
    pub cors: Option<CorsMiddleware>,
    /// Serving static assets
    #[serde(rename = "static")]
    pub static_assets: Option<StaticAssetsMiddleware>,
}

/// Static asset middleware configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticAssetsMiddleware {
    pub enable: bool,
    /// Check that assets must exist on disk
    pub must_exist: bool,
    /// Assets location
    pub folder: FolderAssetsMiddleware,
    /// Fallback page for a case when no asset exists (404). Useful for SPA
    /// (single page app) where routes are virtual.
    pub fallback: String,
    /// Enable `precompressed_gzip`
    #[serde(default = "bool::default")]
    pub precompressed: bool,
}

/// Asset folder config.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FolderAssetsMiddleware {
    /// Uri for the assets
    pub uri: String,
    /// Path for the assets
    pub path: String,
}

/// CORS middleware configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsMiddleware {
    pub enable: bool,
    /// Allow origins
    pub allow_origins: Option<Vec<String>>,
    /// Allow headers
    pub allow_headers: Option<Vec<String>>,
    /// Allow methods
    pub allow_methods: Option<Vec<String>>,
    /// Max age
    pub max_age: Option<u64>,
}

/// Timeout middleware configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeoutRequestMiddleware {
    pub enable: bool,
    // Timeout request in milliseconds
    pub timeout: u64,
}

/// Limit payload size middleware configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LimitPayloadMiddleware {
    pub enable: bool,
    /// Body limit. for example: 5mb
    pub body_limit: String,
}

/// A generic middleware configuration that can be enabled or
/// disabled.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnableMiddleware {
    pub enable: bool,
}

/// Mailer configuration
///
/// Example (development), to capture mails with something like [mailcrab](https://github.com/tweedegolf/mailcrab):
/// ```yaml
/// # config/development.yaml
/// mailer:
///   smtp:
///     enable: true
///     host: localhost
///     port: 1025
///     secure: false
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mailer {
    pub smtp: Option<SmtpMailer>,

    #[serde(default)]
    pub stub: bool,
}

/// Initializers configuration
///
/// Example (development): To configure settings for oauth2 or custom view
/// engine
/// ```yaml
/// # config/development.yaml
/// initializers:
///  oauth2:
///   authorization_code: # Authorization code grant type
///     - client_identifier: google # Identifier for the `OAuth2` provider.
///       Replace 'google' with your provider's name if different, must be
///       unique within the oauth2 config. ... # other fields
pub type Initializers = BTreeMap<String, serde_json::Value>;

/// SMTP mailer configuration structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmtpMailer {
    pub enable: bool,
    /// SMTP host. for example: localhost, smtp.gmail.com etc.
    pub host: String,
    /// SMTP port/
    pub port: u16,
    /// Enable TLS
    pub secure: bool,
    /// Auth SMTP server
    pub auth: Option<MailerAuth>,
}

/// Authentication details for the mailer
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailerAuth {
    /// User
    pub user: String,
    /// Password
    pub password: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub cache_type: String,    
    pub namespace: Option<String>,
    pub url: Option<String>,    
    pub pool_size: Option<u32>,
}
 