#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MutabilityEnumType {
    #[default]
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
