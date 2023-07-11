#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum HashAlgorithmEnumType {
    #[default]
    SHA256,
    SHA384,
    SHA512,
}
