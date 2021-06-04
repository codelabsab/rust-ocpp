#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}
