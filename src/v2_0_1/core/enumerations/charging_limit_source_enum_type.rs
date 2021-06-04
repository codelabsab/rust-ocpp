#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ChargingLimitSourceEnumType {
    EMS,
    Other,
    SO,
    CSO,
}
