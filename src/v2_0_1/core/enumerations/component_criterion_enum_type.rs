#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ComponentCriterionEnumType {
    Active,
    Available,
    Enabled,
    Problem,
}
