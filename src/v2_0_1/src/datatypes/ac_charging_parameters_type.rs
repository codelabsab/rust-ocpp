//! EV AC charging parameters

/// EV AC charging parameters
///
/// ACChargingParametersType is used by: [ChargingNeedsType](`crate::datatypes::charging_needs_type::ChargingNeedsType`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    pub energy_amount: i64,
    /// Minimum current (amps) supported by theelectric vehicle (per phase).
    pub ev_min_current: i64,
    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    pub ev_max_current: i64,
    /// Maximum voltage supported by the electric vehicle
    pub ev_max_voltage: i64,
}
