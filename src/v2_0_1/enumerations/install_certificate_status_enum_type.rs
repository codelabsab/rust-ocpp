#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum InstallCertificateStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Failed,
}
