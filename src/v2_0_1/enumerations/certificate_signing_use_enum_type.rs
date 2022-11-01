#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum CertificateSigningUseEnumType {
    #[default]
    ChargingStationCertificate,
    V2GCertificate,
}
