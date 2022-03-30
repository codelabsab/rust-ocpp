#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
}
