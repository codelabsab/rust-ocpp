#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum DeleteCertificateStatusEnumType {
    #[default]
    Accepted,
    Failed,
    NotFound,
}
