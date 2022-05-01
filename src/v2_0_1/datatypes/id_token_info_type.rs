use chrono::DateTime;
use chrono::Utc;

use super::id_token_type::IdTokenType;
use super::message_content_type::MessageContentType;
use crate::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;

/// Contains status information about an identifier. It is advised to not stop charging for a token that expires during charging, as ExpiryDate is only used for caching purposes. If ExpiryDate is not given, the status has no end date.
/// IdTokenInfoType is used by: Common:AuthorizationData , AuthorizeResponse , TransactionEventResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    pub status: AuthorizationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
}
