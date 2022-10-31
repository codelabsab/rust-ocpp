#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChargingLimitSourceEnumType {
    EMS,
    #[default]
    Other,
    SO,
    CSO,
}
