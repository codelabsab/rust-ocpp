#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    ManufacturerRootCertificate,
}
