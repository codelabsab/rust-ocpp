#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CostKindEnumType {
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}
