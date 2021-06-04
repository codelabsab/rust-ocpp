#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    ManufacturerRootCertificate,
}
