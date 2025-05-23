use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType,
    status_info::StatusInfoType,
};
use crate::v2_1::enumerations::clear_cache_status::ClearCacheStatusEnumType;

/// Request to clear the charging station's cache.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearCacheRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    /// Required. Accepted if the Charging Station has executed the request, otherwise rejected.
    pub status: ClearCacheStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
