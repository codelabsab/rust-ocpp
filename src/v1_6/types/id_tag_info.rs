use chrono::{DateTime, Utc};

use super::AuthorizationStatus;
#[cfg(feature = "std")]
use validator::Validate;

/// Contains status information about an identifier. It is returned in Authorize, Start Transaction and Stop Transaction responses. If expiryDate is not given, the status has no end date.
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo<'a> {
    /// Optional. This contains the date at which idTag should be removed from the Authorization Cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    /// Optional. This contains the parent-identifier. IdToken
    #[cfg_attr(feature="std", validate(length(min = 1, max = 20)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<&'a str>,
    /// Required. This contains whether the idTag has been accepted or not by the Central System.
    pub status: AuthorizationStatus,
}
