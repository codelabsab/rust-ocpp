#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChargingProfileStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
