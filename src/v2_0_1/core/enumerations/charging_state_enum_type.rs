#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ChargingStateEnumType {
    Charging,
    EVConnected,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}
