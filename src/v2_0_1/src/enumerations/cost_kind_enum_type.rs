#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum CostKindEnumType {
    #[default]
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}
