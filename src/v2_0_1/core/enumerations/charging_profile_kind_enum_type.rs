#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Recurring,
    Relative,
}
