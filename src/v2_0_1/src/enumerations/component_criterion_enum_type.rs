#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ComponentCriterionEnumType {
    Active,
    #[default]
    Available,
    Enabled,
    Problem,
}
