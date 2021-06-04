#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum AuthorizeCertificateStatusEnumType {
    Accepted,
    SignatureError,
    CertificateExpired,
    NoCertificateAvailable,
    CertChainError,
    CertificateRevoked,
    ContractCancelled,
}
