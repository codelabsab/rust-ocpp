#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AttributeEnumType {
    #[default]
    Actual,
    Target,
    MinSet,
    MaxSet,
}
