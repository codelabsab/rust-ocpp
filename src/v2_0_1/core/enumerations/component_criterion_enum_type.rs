#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ComponentCriterionEnumType {
    Active,
    Available,
    Enabled,
    Problem,
}
