#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChargingProfileKindEnumType {
    #[default]
    Absolute,
    Recurring,
    Relative,
}
