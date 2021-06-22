#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum NotifyEVChargingNeedsStatusEnumType {
    Accepted,
    Rejected,
    Processing,
}
