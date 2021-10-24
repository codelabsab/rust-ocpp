use crate::v2_0_1::datatypes::log_parameters_type::LogParametersType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::log_enum_type::LogEnumType;
use crate::v2_0_1::enumerations::log_status_enum_type::LogStatusEnumType;

/// This contains the field definition of the GetLogRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    pub log_type: LogEnumType,
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    pub log: LogParametersType,
}

/// This contains the field definition of the GetLogResponse PDU sent by the Charging Station to the CSMS in response to a GetLogRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    pub status: LogStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
