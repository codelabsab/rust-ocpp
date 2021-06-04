#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum AttributeEnumType {
    Actual,
    Target,
    MinSet,
    MaxSet,
}
