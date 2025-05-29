use crate::config::Config;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::HttpMessage;
use tracing::Span;
use tracing_actix_web::{RequestId, RootSpanBuilder};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub struct RequestSpan;

impl RootSpanBuilder for RequestSpan {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let request_id = request.extensions().get::<RequestId>().copied().unwrap();
        tracing::info_span!(
            "HTTP",
            method = %request.method(),
            route = %request.uri().path_and_query().map(|p| p.as_str()).unwrap_or(""),
            ip = %request.connection_info().realip_remote_addr().unwrap_or(""),
            %request_id,
            status_code = tracing::field::Empty,
            error = tracing::field::Empty,
        )
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, actix_web::Error>) {
        // Capture error & status codes
        match &outcome {
            Ok(response) => {
                if let Some(error) = response.response().error() {
                    Self::handle_error(span, error)
                } else {
                    span.record("status_code", response.response().status().as_u16());
                }
            }
            Err(error) => Self::handle_error(span, error),
        };
    }
}

impl RequestSpan {
    fn handle_error(span: Span, error: &actix_web::Error) {
        let response_error = error.as_response_error();

        // pre-formatting errors is a workaround for https://github.com/tokio-rs/tracing/issues/1565
        let display = format!("{}", response_error);
        span.record("error", tracing::field::display(display));

        let status_code = response_error.status_code();
        span.record("status_code", status_code.as_u16());
    }
}

pub fn init(config: &Config) {
    oneline_eyre::install().expect("Could not install logging");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(cfg!(debug_assertions))
        .with_target(false);
    let filter_layer = EnvFilter::new(&config.log_filter);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
