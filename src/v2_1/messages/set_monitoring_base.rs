use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{custom_data::CustomDataType, status_info::StatusInfoType},
    enumerations::{
        generic_device_model_status::GenericDeviceModelStatusEnumType,
        monitoring_base::MonitoringBaseEnumType,
    },
};

/// Request to set the monitoring base at the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Specify which monitoring base will be set.
    pub monitoring_base: MonitoringBaseEnumType,
}

/// Response to a SetMonitoringBaseRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Indicates whether the Charging Station was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl SetMonitoringBaseRequest {
    /// Creates a new `SetMonitoringBaseRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `monitoring_base` - Specify which monitoring base will be set
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringBaseRequest` with optional fields set to `None`
    pub fn new(monitoring_base: MonitoringBaseEnumType) -> Self {
        Self {
            custom_data: None,
            monitoring_base,
        }
    }
}

impl SetMonitoringBaseResponse {
    /// Creates a new `SetMonitoringBaseResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Indicates whether the Charging Station was able to accept the request
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringBaseResponse` with optional fields set to `None`
    pub fn new(status: GenericDeviceModelStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
