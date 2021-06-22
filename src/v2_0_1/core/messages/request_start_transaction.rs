use crate::v2_0_1::core::{
    datatypes::{
        charging_profile_type::ChargingProfileType, id_token_type::IdTokenType,
        status_info_type::StatusInfoType,
    },
    enumerations::request_start_stop_status_enum_type::RequestStartStopStatusEnumType,
};

/// This contains the field definitions of the RequestStartTransactionRequest PDU sent to Charging Station by CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    pub remote_start_id: i64,
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

/// This contains the field definitions of the RequestStartTransactionResponse PDU sent from Charging Station to CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionResponse {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
