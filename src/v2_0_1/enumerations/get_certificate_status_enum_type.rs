#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GetCertificateStatusEnumType {
    #[default]
    Accepted,
    Failed,
}
