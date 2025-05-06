use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use ts_rs::TS;
use uuid::Uuid;
use surrealdb::sql::Datetime;

#[derive(Deserialize, Serialize, Debug, TS, Clone)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_url: String,
    #[ts(type = "string")]
    pub created_at: Datetime,
    #[ts(type = "string")]
    pub updated_at: Datetime,
}



impl From<ClerkJsUser> for User {
    fn from(user: ClerkJsUser) -> Self {
        let email = match &user.email_addresses.first() {
            Some(primary) => primary.email_address.clone(),
            None => {
                if !user.email_addresses.is_empty() {
                    user.email_addresses[0].email_address.clone()
                } else {
                    String::new()
                }
            }
        };

    

        User {
            user_id: user.id.clone(),
            email,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            image_url: user.image_url.clone(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ClerkJsUser {
    pub id: String,
    pub password_enabled: bool,
    pub totp_enabled: bool,
    pub backup_code_enabled: bool,
    pub two_factor_enabled: bool,
    pub banned: bool,
    pub locked: bool,
    #[serde(with = "timestamp_milliseconds")]
    #[ts(type = "number")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "timestamp_milliseconds")]
    #[ts(type = "number")]
    pub updated_at: DateTime<Utc>,
    pub image_url: String,
    pub has_image: bool,
    pub primary_email_address_id: Option<String>,
    pub primary_phone_number_id: Option<String>,
    pub primary_web3_wallet_id: Option<String>,
    #[serde(with = "timestamp_milliseconds_option")]
    #[ts(type = "number | null")]
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub external_id: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub public_metadata: serde_json::Value,
    pub private_metadata: serde_json::Value,
    pub unsafe_metadata: serde_json::Value,
    pub email_addresses: Vec<EmailAddress>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub web3_wallets: Vec<Web3Wallet>,
    pub external_accounts: Vec<ExternalAccount>,
    pub saml_accounts: Vec<SamlAccount>,
    #[serde(with = "timestamp_milliseconds_option")]
    #[ts(type = "number | null")]
    pub last_active_at: Option<DateTime<Utc>>,

    pub create_organization_enabled: bool,
    pub create_organizations_limit: Option<i32>,
    pub delete_self_enabled: bool,
    #[serde(with = "timestamp_milliseconds_option")]
    #[ts(type = "number | null")]
    pub legal_accepted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    pub id: String,
    pub email_address: String,
    pub verification: Option<Verification>,
    #[serde(default)]
    pub linked_to: Vec<IdentificationLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct IdentificationLink {
    pub id: String,
    #[serde(rename = "type")]
    pub link_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PhoneNumber {
    pub id: String,
    pub phone_number: String,
    pub verification: Option<Verification>,
    pub linked_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Verification {
    pub status: String, // "verified" or "unverified"
    pub strategy: String,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ExternalAccount {
    pub id: String,
    pub provider: String, // "google", "github", etc.
    pub identification_id: String,
    pub external_id: String,
    pub approved_scopes: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub image_url: String,
    pub username: Option<String>,
    pub public_metadata: serde_json::Value,
    pub label: Option<String>,
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OrganizationMembership {
    pub id: String,
    pub organization_id: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Web3Wallet {
    pub id: String,
    pub web3_wallet: String,
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SamlAccount {
    pub id: String,
    pub provider: String,
    pub email_address: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub verification: Option<Verification>,
}

mod timestamp_milliseconds {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp_millis();
        serializer.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        Utc.timestamp_millis_opt(timestamp)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
    }
}

mod timestamp_milliseconds_option {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let timestamp = date.timestamp_millis();
                serializer.serialize_some(&timestamp)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp: Option<i64> = Option::deserialize(deserializer)?;
        match timestamp {
            Some(ts) => Utc
                .timestamp_millis_opt(ts)
                .single()
                .map(Some)
                .ok_or_else(|| serde::de::Error::custom("invalid timestamp")),
            None => Ok(None),
        }
    }
}

mod surreal_datetime_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Format the datetime in a way SurrealDB expects
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}