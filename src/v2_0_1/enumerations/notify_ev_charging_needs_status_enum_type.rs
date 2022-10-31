#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum NotifyEVChargingNeedsStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Processing,
}
