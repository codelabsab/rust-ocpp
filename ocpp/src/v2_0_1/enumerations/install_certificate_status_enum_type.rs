#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}
