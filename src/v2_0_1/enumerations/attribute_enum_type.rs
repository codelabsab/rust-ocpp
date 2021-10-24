#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum AttributeEnumType {
    Actual,
    Target,
    MinSet,
    MaxSet,
}
