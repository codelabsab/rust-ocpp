#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Recurring,
    Relative,
}
