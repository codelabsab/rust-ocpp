use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::evse_type::EVSEType;
use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::message_content_type::MessageContentType;
use crate::v2_0_1::datatypes::meter_value_type::MeterValueType;
use crate::v2_0_1::datatypes::transaction_type::TransactionType;
use crate::v2_0_1::enumerations::transaction_event_enum_type::TransactionEventEnumType;
use crate::v2_0_1::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
use crate::Vec;

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest<'a, const N_METER_VALUES: usize, const N_ADDITIONAL_INFOS: usize, const N_SAMPLED_VALUES: usize> {
    pub event_type: TransactionEventEnumType,
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnumType,
    pub seq_no: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i64>,
    #[serde(borrow)]
    pub transaction_info: TransactionType<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType<'a, N_ADDITIONAL_INFOS>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_value: Option<Vec<MeterValueType<'a, N_SAMPLED_VALUES>, N_METER_VALUES>>,
}

/// This contains the field definition of the TransactionEventResponse PDU sent by the CSMS to the Charging Station in response to a TransactionEventRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse<'a, const N_EVSE_IDS: usize, const TOKEN_N_ADDITIONAL_INFOS: usize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub id_token_info: Option<IdTokenInfoType<'a, N_EVSE_IDS, TOKEN_N_ADDITIONAL_INFOS>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType<'a>>,
}
