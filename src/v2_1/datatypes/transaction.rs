use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType};
use crate::v2_1::enumerations::ChargingStateEnumType;

/// A transaction for charging an EV.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the identifier by which the Charging Station and the CSMS identify the transaction.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Optional. The identifier that identifies the current charging state of the charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,

    /// Optional. The time when the transaction was started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_start: Option<DateTime<Utc>>,

    /// Optional. The time when the transaction was stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_end: Option<DateTime<Utc>>,

    /// Optional. This contains the identifier by which the user that started the transaction can be identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    /// Optional. This contains the identifier by which the user that stopped the transaction can be identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_id_token: Option<IdTokenType>,
}
