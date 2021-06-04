#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CostKindEnumType {
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}
