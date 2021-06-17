#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum CertificateSignedStatusEnumType {
    Accepted,
    Rejected,
}
