use actix_web::{get, middleware, web::Data, App, HttpServer};
use biscuit::{Validation, ValidationOptions};
use futures_util::TryFutureExt;
use tokio::sync::watch;
use tracing_actix_web::TracingLogger;

mod auth;
mod bank;
mod config;
mod context;
mod controllers;
mod emulator;
mod error;
mod logging;
mod models;
mod rbac;
mod signer;
mod utils;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[actix_web::main]
async fn main() -> Result<(), eyre::Report> {
    let config = crate::config::Config::new()?;
    logging::init(&config);
    tracing::info!(?config);
    {
        let db_pool = m10_rds_pool::RdsManager::new(config.database_url.clone())
            .pool()
            .await?;
        let mut conn = db_pool.get().await?;
        MIGRATOR.run(&mut *conn).await?;
    }
    let context = context::Context::new_from_config(config.clone()).await?;
    let (jwks_s, jwks_r) = watch::channel(auth::empty_jwks());
    tokio::spawn(auth::watch_jwks(config.oauth.issuer.parse()?, jwks_s));
    let validation_options = ValidationOptions {
        issuer: Validation::Validate(config.oauth.issuer),
        audience: Validation::Validate(config.oauth.audience),
        ..Default::default()
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(Data::new(validation_options.clone()))
            .app_data(Data::new(jwks_r.clone()))
            .app_data(Data::new(context.clone()))
            .wrap(TracingLogger::<logging::RequestSpan>::new())
            .service(healthz)
            .configure(controllers::configure)
    });
    let server = server.bind(config.listen_addr)?;
    server.run().map_err(eyre::Report::from).await
}

#[get("/healthz")]
async fn healthz() -> &'static str {
    "Status OK"
}
