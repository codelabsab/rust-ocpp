#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ChargingProfileStatusEnumType {
    Accepted,
    Rejected,
}
