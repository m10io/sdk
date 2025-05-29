use actix_web::{get, middleware, web::Data, App, HttpServer};
use biscuit::{Validation, ValidationOptions};
use futures_util::TryFutureExt;
use reqwest::Url;
use tokio::sync::watch;
use tracing::{error, info_span, Instrument};
use tracing_actix_web::TracingLogger;

mod auth;
mod bank;
mod cbdc;
mod config;
mod context;
mod controllers;
mod drc;
mod emulator;
mod error;
mod liquidity;
mod logging;
mod models;
mod rbac;
mod requiem;
mod signer;
mod transfer_observer;
mod utils;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"

const TASK_INIT_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(5);

#[tokio::main]
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
    let mut context = context::Context::new_from_config(config.clone()).await?;
    context.init().await?;

    let issuer_uri: Url = config.oauth.issuer.parse()?;
    let (jwks_s, jwks_r) = watch::channel(auth::empty_jwks());
    tokio::spawn(auth::watch_jwks(issuer_uri.clone(), jwks_s));
    let full_issuer_uri = Url::join(&issuer_uri, "/realms/master")?;

    let validation_options = ValidationOptions {
        issuer: Validation::Validate(full_issuer_uri.to_string()),
        ..Default::default()
    };

    // Transfer observer and handler tasks
    let (cbdc_adjustment_tx, cbdc_adjustment_rx) = flume::unbounded();
    let (cbdc_reserve_adjustment_tx, cbdc_reserve_adjustment_rx) = flume::unbounded();
    let (drc_reserve_adjustment_tx, drc_reserve_adjustment_rx) = flume::unbounded();

    let cbdc_balance_observer_task: Vec<tokio::task::JoinHandle<eyre::Result<()>>> = context
        .config
        .currencies
        .values()
        .filter_map(|v| {
            let cc = context.clone();
            v.cbdc_config.as_ref().map(|_| {
                let observer = transfer_observer::TransferObserver::new(
                    cbdc_adjustment_tx.clone(),
                    models::TransferHandler::CbdcLimits,
                    &v.code,
                );
                tokio::spawn(async move {
                    loop {
                        if let Err(err) = observer
                            .run(
                                cc.clone(),
                                cbdc::CbdcAdjustmentHandler::find_matching_transfer,
                            )
                            .instrument(info_span!("cbdc-balance-limit-observer"))
                            .await
                        {
                            error!(%err, "CBDC balance limit observation failed");
                            tokio::time::sleep(TASK_INIT_RETRY_DELAY).await;
                        }
                    }
                })
            })
        })
        .collect();

    let cbdc_adjustment_task = tokio::spawn(
        cbdc::CbdcAdjustmentHandler::new(cbdc_adjustment_rx.into_stream(), context.clone())
            .start()
            .instrument(info_span!("cbdc_adjustment")),
    );

    let cbdc_liquidity_observer_task: Vec<tokio::task::JoinHandle<eyre::Result<()>>> = context
        .config
        .currencies
        .values()
        .filter_map(|v| {
            let cc = context.clone();
            v.cbdc_config.as_ref().map(|_| {
                let observer = transfer_observer::TransferObserver::new(
                    cbdc_reserve_adjustment_tx.clone(),
                    models::TransferHandler::CbdcReserves,
                    &v.code,
                );
                tokio::spawn(async move {
                    loop {
                        if let Err(err) = observer
                            .run(cc.clone(), cbdc::CbdcReserveHandler::find_matching_transfer)
                            .instrument(info_span!("cbdc-liquidity-observer"))
                            .await
                        {
                            error!(%err, "CBDC Liquidity observation failed");
                            tokio::time::sleep(TASK_INIT_RETRY_DELAY).await;
                        }
                    }
                })
            })
        })
        .collect();

    let cbdc_reserve_adjustment_task = tokio::spawn(
        cbdc::CbdcReserveHandler::new(cbdc_reserve_adjustment_rx.into_stream(), context.clone())
            .start()
            .instrument(info_span!("cbdc_reserve_adjustment")),
    );

    let drc_liquidity_observer_task: Vec<tokio::task::JoinHandle<eyre::Result<()>>> = context
        .config
        .currencies
        .values()
        .filter_map(|v| {
            let cc = context.clone();
            v.cbdc_config.as_ref().map(|_| {
                let observer = transfer_observer::TransferObserver::new(
                    drc_reserve_adjustment_tx.clone(),
                    models::TransferHandler::DrcReserves,
                    &v.code,
                );
                tokio::spawn(async move {
                    loop {
                        if let Err(err) = observer
                            .run(cc.clone(), drc::DrcReserveHandler::find_matching_transfer)
                            .instrument(info_span!("drc-transfer-observer"))
                            .await
                        {
                            error!(%err, "DRM Transfer observation failed");
                            tokio::time::sleep(TASK_INIT_RETRY_DELAY).await;
                        }
                    }
                })
            })
        })
        .collect();

    let drc_reserve_adjustment_task = tokio::spawn(
        drc::DrcReserveHandler::new(drc_reserve_adjustment_rx.into_stream(), context.clone())
            .start()
            .instrument(info_span!("drc_reserve_adjustment")),
    );

    let cc = context.clone();
    let requiem_service_init_task = tokio::spawn(async move {
        while let Err(err) = cc
            .requiem
            .init(&cc)
            .instrument(info_span!("requiem-service-init"))
            .await
        {
            error!(%err, "Requiem service initialization failed");
            tokio::time::sleep(TASK_INIT_RETRY_DELAY).await;
            continue;
        }
    });

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
    let http_server_task = tokio::spawn(server.run().map_err(eyre::Report::from));
    tokio::select! {
        res = futures_util::future::join_all(cbdc_balance_observer_task) => { for r in res {
            r??
         }
        },
        res = futures_util::future::join_all(cbdc_liquidity_observer_task) => { for r in res {
            r??
         }
        },
        res = futures_util::future::join_all(drc_liquidity_observer_task) => { for r in res {
            r??
         }
        },
        res = cbdc_adjustment_task => res??,
        res = cbdc_reserve_adjustment_task => res??,
        res = drc_reserve_adjustment_task => res??,
        res = http_server_task => res??
    }

    requiem_service_init_task.await?;

    Ok(())
}

#[get("/healthz")]
async fn healthz() -> &'static str {
    "Status OK"
}
