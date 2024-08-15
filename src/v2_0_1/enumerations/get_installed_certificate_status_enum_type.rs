#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub enum GetInstalledCertificateStatusEnumType {
    #[default]
    Accepted,
    NotFound,
}
