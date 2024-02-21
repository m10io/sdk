use actix_web::{get, web::Data, Scope};

use crate::{context::Context, error::Error};

use m10_sdk::Signer;

#[get("")]
async fn get(context: Data<Context>) -> Result<String, Error> {
    Ok(base64::encode(context.ledger.signer()?.public_key()))
}

pub fn scope() -> Scope {
    actix_web::web::scope("public_key").service(get)
}
