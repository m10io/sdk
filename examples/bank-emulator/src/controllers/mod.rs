use actix_web::web::ServiceConfig;

mod accounts;
mod assets;
mod contacts;
mod documents;
mod fees;
mod notification_preferences;
mod public_key;
mod transfer_methods;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        actix_web::web::scope("/api/v1")
            .service(accounts::scope())
            .service(assets::scope())
            .service(contacts::scope())
            .service(documents::scope())
            .service(fees::scope())
            .service(notification_preferences::scope())
            .service(public_key::scope())
            .service(transfer_methods::scope()),
    );
}
