#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum NotifyEVChargingNeedsStatusEnumType {
    Accepted,
    Rejected,
    Processing,
}
