/// Charging schedule period structure defines a time period in a charging schedule
/// ChargingSchedulePeriodType is used by: Common:ChargingScheduleType , Common:CompositeScheduleType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    pub start_period: i64,
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i64>,
}
