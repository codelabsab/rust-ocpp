#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}
