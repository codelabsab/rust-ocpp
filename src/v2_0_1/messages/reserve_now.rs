use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::connector_enum_type::ConnectorEnumType;
use crate::v2_0_1::enumerations::reserve_now_status_enum_type::ReserveNowStatusEnumType;

/// This contains the field definitions of the RequestStopTransactionRequest PDU sent to Charging Station by CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    pub id: i64,
    pub expiry_date_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

/// This contains the field definition of the ReserveNowResponse PDU sent by the Charging Station to the CSMS in response toReserveNowRequest PDU.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowResponse {
    pub status: ReserveNowStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
