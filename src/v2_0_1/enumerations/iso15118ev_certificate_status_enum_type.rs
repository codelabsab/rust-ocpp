#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Iso15118EVCertificateStatusEnumType {
    Accepted,
    Failed,
}
