use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;

/// ChargingLimitType is used by: NotifyChargingLimitRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    /// Required. Represents the source of the charging limit.
    pub charging_limit_source: ChargingLimitSourceEnumType,
    /// Optional. Indicates whether the charging limit is critical for the grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,
}
