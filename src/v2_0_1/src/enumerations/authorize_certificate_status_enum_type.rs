#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AuthorizeCertificateStatusEnumType {
    #[default]
    Accepted,
    SignatureError,
    CertificateExpired,
    NoCertificateAvailable,
    CertChainError,
    CertificateRevoked,
    ContractCancelled,
}
