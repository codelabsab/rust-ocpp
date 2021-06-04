#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GetCertificateIdUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
}
