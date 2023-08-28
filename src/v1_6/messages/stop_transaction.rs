use crate::v1_6::types::{IdTagInfo, MeterValue, Reason};
use crate::Vec;

use chrono::{DateTime, Utc};
#[cfg(feature = "std")]
use validator::Validate;

/// This contains the field definition of the StopTransaction.req PDU sent by the Charge Point to the Central System. See also Stop Transaction
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionRequest<'a, const N_TXN_DATA: usize, const N_METER_VALUES_PER_TXN: usize> {
    /// Required.
    #[cfg_attr(feature="std", validate(length(min = 1, max = 20)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<&'a str>, // IdToken, should this be a type?
    /// Optional. Only filled in when request applies to a specific connector.
    pub meter_stop: i64,
    /// Required. This contains the date and time on which the transaction is stopped.
    pub timestamp: DateTime<Utc>,
    /// Required. This contains the transaction-id as received by the StartTransactionResponse
    pub transaction_id: i64,
    /// Optional. This contains the reason why the transaction was stopped. MAY only be omitted when the Reason is "Local".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    /// Optional. This contains transaction usage details relevant for billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<MeterValue<'a, N_METER_VALUES_PER_TXN>, N_TXN_DATA>>,
}

/// This contains the field definition of the TriggerMessage.req PDU sent by the Central System to the Charge Point. See also Trigger Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse<'a> {
    /// Optional. This contains information about authorization status, expiry and parent id. It is optional, because a transaction may have been stopped without an identifier.
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub id_tag_info: Option<IdTagInfo<'a>>,
}
