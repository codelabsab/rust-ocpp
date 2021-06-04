use crate::v2_0_1::core::{
    datatypes::evse_type::EVSEType,
    enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType,
};

/// This contains the field definition of the ClearedChargingLimitRequest PDU sent by the Charging Station to the CSMS
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

/// This contains the field definition of the ClearedChargingLimitResponse PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {}
