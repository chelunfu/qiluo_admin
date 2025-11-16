use crate::api::WebApi;
use crate::config::appconfig::LogLevel;
use crate::config::APPCOFIG;
use crate::midle_ware::jwt::UserInfo;

use axum::middleware;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Router};
use axum_server::tls_rustls::RustlsConfig;

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio::signal;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::predicate::NotForContentType;
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{
    fmt::{self, time},
    EnvFilter, Registry,
};

use crate::cache::CacheManager;
use crate::midle_ware::{ApiMid, AuthMid, OperateLogMid, RequestLogMid};
use crate::worker::processor_job;
use crate::DB;

use crate::service::sys::s_sys_job;

#[derive(Default)]
pub struct App;

impl App {
    pub fn app_version() -> String {
        "QiLuo 2.0".to_string()
    }
    pub fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }
    pub async fn db_init(self) -> Self {
        DB().await;
        CacheManager::init()
            .await
            .expect("Failed to initialize cache manager");
        self.worker_init().await
    }
    pub async fn worker_init(self) -> Self {
        let p = processor_job().await;
        tokio::spawn(async move {
            p.run().await;
        });
        s_sys_job::update_job().await;
        self
    }
    pub async fn run() {
        let lconfig = APPCOFIG.logger.clone();

        let file_appender = tracing_appender::rolling::hourly(lconfig.log_dir, lconfig.file_name);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

        let get_level = match lconfig.level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        };

        let format = fmt::format()
            .with_level(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_timer(time::LocalTime::rfc_3339())
            .compact();

        let logger = Registry::default()
            .with(EnvFilter::from_default_env().add_directive(get_level.into()))
            .with(
                fmt::Layer::default()
                    .with_writer(std_non_blocking)
                    .event_format(format.clone())
                    .pretty(),
            )
            .with(
                fmt::Layer::default()
                    .with_writer(non_blocking)
                    .event_format(format),
            );

        tracing::subscriber::set_global_default(logger).expect("Unable to set global subscriber");
        let mut app = App;
        app = app.db_init().await;
        let _ = app.start().await;
    }

    fn routes(&self) -> Router {
        let serverconfig = APPCOFIG.server.clone();
        let staticdir = ServeDir::new(serverconfig.static_dir);
        let webdir = ServeDir::new(serverconfig.web_dir);
        Router::new() 
            .nest_service("/static", staticdir)
            .nest("/api", self.set_auth_middleware(WebApi::routers()))
            .nest("/api", WebApi::white_routers())
            .fallback_service(webdir)
            .layer(middleware::from_fn(RequestLogMid))
    }

    pub fn set_auth_middleware(&self, router: Router) -> Router {
        router
            .layer(middleware::from_fn(OperateLogMid))
            .layer(middleware::from_fn(ApiMid))
            .layer(middleware::from_fn(AuthMid))
            .layer(middleware::from_extractor::<UserInfo>())
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let serverconfig = APPCOFIG.server.clone();
        let addr = format!("{}:{}", serverconfig.binding, serverconfig.port);

        tracing::info!(
            "Server is running on {}:{}",
            serverconfig.host,
            serverconfig.port
        );
        let app = self.routes();
        let app = self.configure_middlewares(app);

        if serverconfig.ssl.enable {
            self.start_https_server(app, &addr).await
        } else {
            self.start_http_server(app, &addr).await
        }
    }

    async fn start_https_server(
        &self,
        app: Router,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serverconfig = APPCOFIG.server.clone();
        let config = RustlsConfig::from_pem_file(serverconfig.ssl.cert, serverconfig.ssl.key)
            .await
            .map_err(|e| format!("Failed to load TLS config: {}", e))?;

        let socket_addr = SocketAddr::from_str(addr)
            .map_err(|e| format!("Failed to parse socket address: {}", e))?;

        let handle = axum_server::Handle::new();

        tokio::spawn({
            let handle = handle.clone();
            async move {
                App::shutdown_signal().await;
                handle.graceful_shutdown(Some(Duration::from_secs(30)));
            }
        });

        tracing::info!("Starting HTTPS server on {}", addr);
        axum_server::bind_rustls(socket_addr, config)
            .handle(handle)
            .serve(app.into_make_service())
            .await
            .map_err(|e| format!("HTTPS server error: {}", e))?;

        Ok(())
    }

    async fn start_http_server(
        &self,
        app: Router,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| format!("Failed to bind to address: {}", e))?;

        tracing::info!("Starting HTTP server on {}", addr);
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(App::shutdown_signal())
            .await
            .map_err(|e| format!("HTTP server error: {}", e))?;

        Ok(())
    }

    fn configure_cors(&self, serverconfig: &crate::config::appconfig::Server) -> CorsLayer {
        let mut cors = CorsLayer::new();

        if let Some(corsconfig) = serverconfig.middlewares.cors.clone() {
            if corsconfig.enable {
                if let Some(allow_origins) = corsconfig.allow_origins {
                    let origins: Result<Vec<_>, _> = allow_origins
                        .into_iter()
                        .map(|origin| origin.parse())
                        .collect();
                    if let Ok(origins) = origins {
                        cors = cors.allow_origin(origins);
                    }
                }

                if let Some(allow_headers) = corsconfig.allow_headers {
                    let headers: Result<Vec<_>, _> = allow_headers
                        .into_iter()
                        .map(|header| header.parse())
                        .collect();
                    if let Ok(headers) = headers {
                        cors = cors.allow_headers(headers);
                    }
                }

                if let Some(allow_methods) = corsconfig.allow_methods {
                    let methods: Result<Vec<_>, _> = allow_methods
                        .into_iter()
                        .map(|method| method.parse())
                        .collect();
                    if let Ok(methods) = methods {
                        cors = cors.allow_methods(methods);
                    }
                }

                if let Some(max_age) = corsconfig.max_age {
                    cors = cors.max_age(Duration::from_secs(max_age));
                }
            }
        }

        cors.allow_credentials(true)
    }

    fn configure_middlewares(&self, mut app: Router) -> Router {
        let serverconfig = APPCOFIG.server.clone();

        // CORS配置
        app = app.layer(self.configure_cors(&serverconfig));

        // Payload限制
        if let Some(limit) = serverconfig.middlewares.limit_payload.clone() {
            if limit.enable {
                if let Ok(size) = byte_unit::Byte::parse_str(&limit.body_limit, true) {
                    app = app.layer(axum::extract::DefaultBodyLimit::max(size.as_u64() as usize));
                    tracing::info!(
                        data = &limit.body_limit,
                        "[Middleware] Adding limit payload"
                    );
                }
            }
        }

        // Panic处理
        if let Some(catch_panic) = serverconfig.middlewares.catch_panic {
            if catch_panic.enable {
                app = app.layer(CatchPanicLayer::custom(Self::handle_panic));
            }
        }

        // 压缩
        if let Some(compression) = serverconfig.middlewares.compression.clone() {
            if compression.enable {
                let predicate =
                    DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
                app = app.layer(CompressionLayer::new().compress_when(predicate));
                tracing::info!("[Middleware] Adding compression layer");
            }
        }

        // 超时
        if let Some(timeout_request) = serverconfig.middlewares.timeout_request {
            if timeout_request.enable {
                app = app.layer(TimeoutLayer::new(Duration::from_millis(
                    timeout_request.timeout,
                )));
                tracing::info!("[Middleware] Adding timeout layer");
            }
        }

        app
    }
    #[allow(clippy::needless_pass_by_value)]
    fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> axum::response::Response {
        let err = err.downcast_ref::<String>().map_or_else(
            || err.downcast_ref::<&str>().map_or("no error details", |s| s),
            |s| s.as_str(),
        );

        tracing::error!(err.msg = err, "server_panic");

        (StatusCode::INTERNAL_SERVER_ERROR, "message".to_string()).into_response()
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        info!("signal received, starting graceful shutdown");
    }
}
