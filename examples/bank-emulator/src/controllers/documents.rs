use actix_multipart::Multipart;
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Scope,
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
async fn upload(
    _request: Multipart,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[get("{id}")]
async fn get(
    _ids: Path<String>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<serde_json::Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[patch("{id}")]
async fn update(
    _ids: Path<String>,
    _request: Json<Value>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<serde_json::Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/sandbox/verify")]
async fn verify(
    _ids: Path<String>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<HttpResponse, Error> {
    Err(Error::internal_msg("unimplemented"))
}

pub fn scope() -> Scope {
    actix_web::web::scope("documents")
        .service(list)
        .service(upload)
        .service(get)
        .service(update)
        .service(verify)
}
