#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum CertificateActionEnumType {
    #[default]
    Install,
    Update,
}
