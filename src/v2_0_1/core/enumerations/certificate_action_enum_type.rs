#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CertificateActionEnumType {
    Install,
    Update,
}
