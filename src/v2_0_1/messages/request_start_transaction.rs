use crate::v2_0_1::datatypes::charging_profile_type::ChargingProfileType;
use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::request_start_stop_status_enum_type::RequestStartStopStatusEnumType;

/// This contains the field definitions of the RequestStartTransactionRequest PDU sent to Charging Station by CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    pub remote_start_id: i64,
    #[serde(borrow)]
    pub id_token: IdTokenType<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType<'a>>,
}

/// This contains the field definitions of the RequestStartTransactionResponse PDU sent from Charging Station to CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionResponse<'a> {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType<'a>>,
}
