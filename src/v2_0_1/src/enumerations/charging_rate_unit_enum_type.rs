#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChargingRateUnitEnumType {
    W,
    #[default]
    A,
}
