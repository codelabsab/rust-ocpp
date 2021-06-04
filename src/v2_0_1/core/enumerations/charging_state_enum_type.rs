#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ChargingStateEnumType {
    Charging,
    EVConnected,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}
