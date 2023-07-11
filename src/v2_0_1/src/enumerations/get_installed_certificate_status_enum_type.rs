#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum GetInstalledCertificateStatusEnumType {
    #[default]
    Accepted,
    NotFound,
}
