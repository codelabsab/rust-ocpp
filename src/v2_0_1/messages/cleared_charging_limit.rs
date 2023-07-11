//! ClearedChargingLimit
use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;

/// ClearedChargingLimitRequest, sent by the Charging Station to the CSMS
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    /// Source of the charging limit.
    pub charging_limit_source: ChargingLimitSourceEnumType,
    /// EVSE Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

/// ClearedChargingLimitResponse, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {
    // No fields are defined.
}
