#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum AuthorizeCertificateStatusEnumType {
    Accepted,
    SignatureError,
    CertificateExpired,
    NoCertificateAvailable,
    CertChainError,
    CertificateRevoked,
    ContractCancelled,
}
