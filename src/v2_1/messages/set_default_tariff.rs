use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{custom_data::CustomDataType, status_info::StatusInfoType, tariff::TariffType},
    enumerations::tariff_set_status::TariffSetStatusEnumType,
};

/// Request to set a default tariff at the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. EVSE that tariff applies to. When evseId = 0, then tariff applies to all EVSEs.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. The tariff to be set at the Charging Station.
    pub tariff: TariffType,
}

/// Response to a SetDefaultTariffRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: TariffSetStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl SetDefaultTariffRequest {
    /// Creates a new `SetDefaultTariffRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - EVSE that tariff applies to
    /// * `tariff` - The tariff to be set at the Charging Station
    ///
    /// # Returns
    ///
    /// A new instance of `SetDefaultTariffRequest` with optional fields set to `None`
    pub fn new(evse_id: i32, tariff: TariffType) -> Self {
        Self {
            custom_data: None,
            evse_id,
            tariff,
        }
    }
}

impl SetDefaultTariffResponse {
    /// Creates a new `SetDefaultTariffResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the Charging Station accepts the request
    ///
    /// # Returns
    ///
    /// A new instance of `SetDefaultTariffResponse` with optional fields set to `None`
    pub fn new(status: TariffSetStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
