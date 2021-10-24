#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ChargingLimitSourceEnumType {
    EMS,
    Other,
    SO,
    CSO,
}
