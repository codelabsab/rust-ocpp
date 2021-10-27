/// EV AC charging parameters. ACChargingParametersType is used by: Common:ChargingNeedsType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// Required. Amount of energy requested (in Wh). This includes energy required for preconditioning.
    pub energy_amount: i64,
    /// Required. Minimum current (amps) supported by theelectric vehicle (per phase).
    pub ev_min_current: i64,
    /// Required. Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    pub ev_max_current: i64,
    /// Required. Maximum voltage supported by the electric vehicle
    pub ev_max_voltage: i64,
}
