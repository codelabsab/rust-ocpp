#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
}
