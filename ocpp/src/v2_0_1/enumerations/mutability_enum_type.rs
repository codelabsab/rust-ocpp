#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MutabilityEnumType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
