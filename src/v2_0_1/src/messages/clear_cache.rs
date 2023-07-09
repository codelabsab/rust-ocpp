//! ClearCache
use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType;

/// `ClearCacheRequest`, sent by the CSMS to the Charging Station. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

/// `ClearCacheResponse`, sent by the Charging Station to the CSMS in response to a [`ClearCacheRequest`].
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    /// Accepted if the Charging Station has executed the request, otherwise rejected.
    pub status: ClearCacheStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
