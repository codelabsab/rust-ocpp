use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType,
    status_info::StatusInfoType,
    periodic_event_stream_params::PeriodicEventStreamParamsType,
};
use crate::v2_1::enumerations::generic_status::GenericStatusEnumType;

/// Request body for the AdjustPeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier of the periodic event stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Required. Parameters for the periodic event stream.
    #[validate(nested)]
    pub params: PeriodicEventStreamParamsType,
}

/// Response body for the AdjustPeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
