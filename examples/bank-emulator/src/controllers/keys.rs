use actix_web::{put, web::Data, HttpResponse, Scope};

use crate::{
    auth::{AuthModel, User, Verb},
    context::Context,
    error::Error,
    models::{Contact, ContactAuth},
    rbac,
};

#[put("")]
async fn add(
    request: String,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    ContactAuth.is_authorized(Verb::Update, &current_user)?;
    let mut conn = context.db_pool.get().await?;
    let contact = Contact::find_by_user_id(&current_user.auth0_id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;

    let key = base64::decode(request)?;

    if !rbac::is_key_known(contact.rbac_role, &key, &context).await? {
        rbac::add_key(contact.rbac_role, &key, &context).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn scope() -> Scope {
    actix_web::web::scope("keys").service(add)
}
