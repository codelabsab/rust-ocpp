#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetCertificateStatusEnumType {
    Accepted,
    Failed,
}
