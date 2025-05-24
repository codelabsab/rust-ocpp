use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        authorization_data::AuthorizationData, custom_data::CustomDataType,
        status_info::StatusInfoType,
    },
    enumerations::{send_local_list_status::SendLocalListStatusEnumType, update::UpdateEnumType},
};

/// Request to send a local authorization list to the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. List of authorization data to update in the Local Authorization List.
    /// If empty and updateType is Full, the Local Authorization List will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,

    /// Required. In case of a full update this is the version number of the full list.
    /// In case of a differential update it is the version number of the list after the update has been applied.
    pub version_number: i32,

    /// Required. This contains the type of update (full or differential) of this request.
    pub update_type: UpdateEnumType,
}

/// Response to a SendLocalListRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This indicates whether the Charging Station has successfully received and applied
    /// the update of the Local Authorization List.
    pub status: SendLocalListStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl SendLocalListRequest {
    /// Creates a new `SendLocalListRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `version_number` - Version number of the list
    /// * `update_type` - Type of update (full or differential)
    ///
    /// # Returns
    ///
    /// A new instance of `SendLocalListRequest` with optional fields set to `None`
    pub fn new(version_number: i32, update_type: UpdateEnumType) -> Self {
        Self {
            custom_data: None,
            local_authorization_list: None,
            version_number,
            update_type,
        }
    }
}

impl SendLocalListResponse {
    /// Creates a new `SendLocalListResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Indicates whether the Charging Station has successfully received and applied the update
    ///
    /// # Returns
    ///
    /// A new instance of `SendLocalListResponse` with optional fields set to `None`
    pub fn new(status: SendLocalListStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
