use actix_web::{put, web::Data, HttpResponse, Scope};

use crate::{
    auth::{BankEmulatorRole, User},
    context::Context,
    error::Error,
    models::Contact,
    rbac,
};

#[put("")]
async fn add(
    request: String,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    current_user.is_authorized(BankEmulatorRole::Update)?;
    let mut conn = context.db_pool.get().await?;
    let contact = Contact::find_by_user_id(&current_user.user_id)
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
