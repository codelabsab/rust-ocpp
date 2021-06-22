/// ACChargingParametersType is used by: Common:ChargingNeedsType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    pub energy_amount: i64,
    pub ev_min_current: i64,
    pub ev_max_current: i64,
    pub ev_max_voltage: i64,
}
