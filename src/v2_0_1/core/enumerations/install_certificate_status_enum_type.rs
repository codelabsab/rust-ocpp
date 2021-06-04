#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}
