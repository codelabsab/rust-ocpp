#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MutabilityEnumType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
