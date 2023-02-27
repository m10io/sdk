use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgArguments, PgTypeInfo, PgValueRef},
    query::QueryAs,
    Decode, Encode, Executor, Postgres,
};

use crate::{auth::AuthScope, error::Error};

const SERVICE_INFO_BIT_MASK: u32 = 1;
const IS_ENABLED_BIT_MASK: u32 = 1 << 1;
const TRANSFER_REQUESTED_BIT_MASK: u32 = 1 << 14;
const ACCOUNT_ACTIONS_BIT_MASK: u32 = 1 << 29;
const ACCOUNT_CHANGES_BIT_MASK: u32 = 1 << 30;
const ACCOUNT_TRANSFERS_BIT_MASK: u32 = 1 << 31;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListNotificationPreferencesFilter {
    pub device_token: Option<String>,
    pub instrument: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListNotificationPreferencesResponse {
    pub data: Vec<NotificationPreferences>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<isize>,
}

#[derive(sqlx::FromRow, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub id: i32,

    pub device_token: String,

    pub notification_toggles: NotificationToggles,

    #[serde(skip)]
    pub contacts_id: i64,

    #[serde(skip)]
    pub asset_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationToggles {
    pub service_info: bool,
    pub is_enabled: bool,
    pub transfer_requested: bool,
    pub account_changes: bool,
    pub account_transfers: bool,
    pub account_actions: bool,
}

impl sqlx::Type<Postgres> for NotificationToggles {
    fn type_info() -> PgTypeInfo {
        <i32 as sqlx::Type<Postgres>>::type_info()
    }
}

impl Decode<'_, Postgres> for NotificationToggles
where
    i32: for<'a> Decode<'a, Postgres>,
{
    fn decode(
        value: PgValueRef<'_>,
    ) -> Result<NotificationToggles, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <i32 as Decode<Postgres>>::decode(value)?;
        Ok(value.into())
    }
}

impl Encode<'_, Postgres> for NotificationToggles
where
    i32: for<'a> Encode<'a, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let value: i32 = self.into();
        buf.extend(value.to_be_bytes());

        IsNull::No
    }
}

impl From<&NotificationToggles> for i32 {
    #[inline]
    fn from(toggles: &NotificationToggles) -> i32 {
        let mut res = 0;
        if toggles.service_info {
            res |= SERVICE_INFO_BIT_MASK;
        }
        if toggles.is_enabled {
            res |= IS_ENABLED_BIT_MASK;
        }
        if toggles.transfer_requested {
            res |= TRANSFER_REQUESTED_BIT_MASK;
        }
        if toggles.account_changes {
            res |= ACCOUNT_CHANGES_BIT_MASK;
        }
        if toggles.account_transfers {
            res |= ACCOUNT_TRANSFERS_BIT_MASK;
        }
        if toggles.account_actions {
            res |= ACCOUNT_ACTIONS_BIT_MASK;
        }
        res as i32
    }
}

impl From<i32> for NotificationToggles {
    #[inline]
    fn from(toggles: i32) -> NotificationToggles {
        NotificationToggles {
            service_info: NotificationToggles::is_interested_in(toggles, SERVICE_INFO_BIT_MASK),
            is_enabled: NotificationToggles::is_interested_in(toggles, IS_ENABLED_BIT_MASK),
            transfer_requested: NotificationToggles::is_interested_in(
                toggles,
                TRANSFER_REQUESTED_BIT_MASK,
            ),
            account_changes: NotificationToggles::is_interested_in(
                toggles,
                ACCOUNT_CHANGES_BIT_MASK,
            ),
            account_transfers: NotificationToggles::is_interested_in(
                toggles,
                ACCOUNT_TRANSFERS_BIT_MASK,
            ),
            account_actions: NotificationToggles::is_interested_in(
                toggles,
                ACCOUNT_ACTIONS_BIT_MASK,
            ),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateNotificationPreferencesRequest {
    pub device_token: String,

    pub notification_toggles: NotificationToggles,
}

impl From<CreateNotificationPreferencesRequest> for NotificationPreferences {
    fn from(value: CreateNotificationPreferencesRequest) -> Self {
        let CreateNotificationPreferencesRequest {
            device_token,
            notification_toggles,
        } = value;
        Self {
            device_token,
            notification_toggles,
            ..Default::default()
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateNotificationPreferencesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_toggles: Option<NotificationToggles>,
}

impl NotificationPreferences {
    pub fn find_scoped(
        filter: ListNotificationPreferencesFilter,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, Self, PgArguments>, Error> {
        let ListNotificationPreferencesFilter {
            device_token,
            instrument,
        } = filter;
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "SELECT n.*
                FROM
                    notification_preferences n
                INNER JOIN assets a ON asset_id = a.id    
                WHERE
                    ($1::VARCHAR IS NULL OR device_token = $1::VARCHAR) AND
                    ($2 IS NULL OR (a.instrument = $2 AND asset_id = a.id )) AND 
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            n.contacts_id = c.id AND c.user_id = $3
                )",
            )
            .bind(device_token)
            .bind(instrument.map(|i| i.to_lowercase()))
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "SELECT n.*
                FROM
                    notification_preferences n
                INNER JOIN assets a ON asset_id = a.id
                WHERE
                    ($1::VARCHAR IS NULL OR device_token = $1::VARCHAR) AND
                    ($2 IS NULL OR (a.instrument = $2 AND asset_id = a.id )) AND 
                    tenant = $3",
            )
            .bind(device_token)
            .bind(instrument)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_id_scoped(
        id: i32,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "SELECT *
                FROM
                    notification_preferences
                WHERE
                    id = $1 AND 
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            contacts_id = c.id AND c.user_id = $2
                )",
            )
            .bind(id)
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "SELECT *
                FROM
                    notification_preferences
                WHERE
                    id = $1 AND 
                    tenant = $2",
            )
            .bind(id)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub async fn insert(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "INSERT INTO notification_preferences (
                device_token,
                notification_toggles,
                contacts_id,
                asset_id
            )
            SELECT $1, $2, $3, $4
            WHERE
                EXISTS (
                    SELECT
                        1
                    FROM
                        assets a, contacts c
                    WHERE
                        a.id = $4 AND c.id = $3 AND a.linked_account = c.account_id
                )
            ON CONFLICT ON CONSTRAINT notification_preferences_device_token_asset_id_key DO UPDATE 
                SET notification_toggles = $2, updated_at = CURRENT_TIMESTAMP
            RETURNING *;",
        )
        .bind(&self.device_token)
        .bind(&self.notification_toggles)
        .bind(self.contacts_id)
        .bind(self.asset_id);
        let np = query.fetch_one(txn).await?;
        *self = np;
        Ok(())
    }

    pub async fn update(
        &mut self,
        db: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query(
            "UPDATE
                    notification_preferences
                 SET 
                    notification_toggles = $2,
                    device_token = $3,
                    updated_at = $4
                 WHERE
                    id = $1",
        )
        .bind(self.id)
        .bind(&self.notification_toggles)
        .bind(&self.device_token)
        .bind(self.updated_at);
        query.execute(db).await?;
        Ok(())
    }
}

impl NotificationToggles {
    #[inline]
    fn is_interested_in(toggles: i32, event: u32) -> bool {
        (toggles as u32 & event) != 0
    }

    #[inline]
    fn _is_interested_in_and_enabled(toggles: i32, event: u32) -> bool {
        NotificationToggles::is_interested_in(toggles, event)
            && NotificationToggles::is_interested_in(toggles, IS_ENABLED_BIT_MASK)
    }
}
