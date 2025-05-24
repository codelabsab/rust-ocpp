use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        custom_data::CustomDataType, evse::EVSEType, id_token::IdTokenType,
        id_token_info::IdTokenInfoType, message_content::MessageContentType,
        meter_value::MeterValueType, transaction::TransactionType,
        transaction_limit::TransactionLimitType,
    },
    enumerations::{
        transaction_event::TransactionEventEnumType, trigger_reason::TriggerReasonEnumType,
    },
};

/// Request to notify the CSMS about a transaction event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of event for a transaction.
    pub event_type: TransactionEventEnumType,

    /// Required. List of meter values.
    #[validate(length(min = 1))]
    pub meter_value: Vec<MeterValueType>,

    /// Required. Timestamp of the event.
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Required. Reason the message was triggered.
    pub trigger_reason: TriggerReasonEnumType,

    /// Required. Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Required. Transaction details.
    pub transaction_info: TransactionType,

    /// Optional. Whether the charging station was offline at the time of the transaction event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,

    /// Optional. Number of phases used during the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1, max = 3))]
    pub number_of_phases_used: Option<i32>,

    /// Optional. Maximum current of the connected cable in amperes (A).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub cable_max_current: Option<i32>,

    /// Optional. Reservation ID associated with the transaction, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub reservation_id: Option<i32>,

    /// Optional. EVSE details where the transaction takes place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,

    /// Optional. ID Token used to start or stop the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
}

/// Response to a TransactionEventRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Total cost of the transaction so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,

    /// Optional. Priority of charging from 1 (highest) to 3 (lowest).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1, max = 3))]
    pub charging_priority: Option<i32>,

    /// Optional. Information about the authorization status, expiry and parent ID token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,

    /// Optional. Transaction limits imposed by the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_limit: Option<TransactionLimitType>,

    /// Optional. Updated personal message for the EV driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,

    /// Optional. Additional updated personal messages for the EV driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message_extra: Option<Vec<MessageContentType>>,
}

impl TransactionEventRequest {
    /// Creates a new `TransactionEventRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Type of event for a transaction
    /// * `meter_value` - List of meter values
    /// * `timestamp` - Timestamp of the event
    /// * `trigger_reason` - Reason the message was triggered
    /// * `seq_no` - Sequence number of this message
    /// * `transaction_info` - Transaction details
    ///
    /// # Returns
    ///
    /// A new instance of `TransactionEventRequest` with optional fields set to `None`
    pub fn new(
        event_type: TransactionEventEnumType,
        meter_value: Vec<MeterValueType>,
        timestamp: chrono::DateTime<chrono::Utc>,
        trigger_reason: TriggerReasonEnumType,
        seq_no: i32,
        transaction_info: TransactionType,
    ) -> Self {
        Self {
            custom_data: None,
            event_type,
            meter_value,
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            evse: None,
            id_token: None,
        }
    }
}

impl TransactionEventResponse {
    /// Creates a new `TransactionEventResponse`.
    ///
    /// # Returns
    ///
    /// A new instance of `TransactionEventResponse` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            transaction_limit: None,
            updated_personal_message: None,
            updated_personal_message_extra: None,
        }
    }
}
