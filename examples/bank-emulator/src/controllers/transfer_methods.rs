use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    Scope,
};
use serde_json::Value;

use crate::{
    auth::User,
    context::Context,
    error::Error,
    models::{ListResponse, Page},
};

#[get("")]
async fn list(
    _page: Query<Page<String>>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<ListResponse<String, Value>>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("")]
async fn create(
    _request: Json<Value>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[get("{id}")]
async fn get(
    _id: Path<String>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/deactivate")]
async fn deactivate(
    _id: Path<String>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

pub fn scope() -> Scope {
    actix_web::web::scope("transfer_methods")
        .service(create)
        .service(deactivate)
        .service(list)
        .service(get)
}
