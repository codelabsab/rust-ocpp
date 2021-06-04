#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum HashAlgorithmEnumType {
    SHA256,
    SHA384,
    SHA512,
}
