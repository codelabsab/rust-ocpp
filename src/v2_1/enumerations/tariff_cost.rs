#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TariffCostEnumType {
    #[default]
    #[serde(rename = "Relative")]
    Relative,
    #[serde(rename = "Absolute")]
    Absolute,
    #[serde(rename = "CarbonDioxideEmission")]
    CarbonDioxideEmission,
}
