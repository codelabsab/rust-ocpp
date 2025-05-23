use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        custom_data::CustomDataType, network_connection_profile::NetworkConnectionProfileType,
        status_info::StatusInfoType,
    },
    enumerations::set_network_profile_status::SetNetworkProfileStatusEnumType,
};

/// Request to configure network connection profiles on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Slot in which the configuration should be stored.
    #[validate(range(min = 0))]
    pub configuration_slot: i32,

    /// Required. Network connection profile details.
    pub connection_data: NetworkConnectionProfileType,
}

/// Response to a SetNetworkProfileRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Result of operation.
    pub status: SetNetworkProfileStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl SetNetworkProfileRequest {
    /// Creates a new `SetNetworkProfileRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `configuration_slot` - Slot in which the configuration should be stored
    /// * `connection_data` - Network connection profile details
    ///
    /// # Returns
    ///
    /// A new instance of `SetNetworkProfileRequest` with optional fields set to `None`
    pub fn new(configuration_slot: i32, connection_data: NetworkConnectionProfileType) -> Self {
        Self {
            custom_data: None,
            configuration_slot,
            connection_data,
        }
    }
}

impl SetNetworkProfileResponse {
    /// Creates a new `SetNetworkProfileResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Result of operation
    ///
    /// # Returns
    ///
    /// A new instance of `SetNetworkProfileResponse` with optional fields set to `None`
    pub fn new(status: SetNetworkProfileStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
