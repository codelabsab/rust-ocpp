use crate::v2_0_1::core::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;

/// ChargingLimitType is used by: NotifyChargingLimitRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,
}
