#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GetCertificateIdUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    #[default]
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
}
