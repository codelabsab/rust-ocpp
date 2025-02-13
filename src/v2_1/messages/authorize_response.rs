use super::super::datatypes::{CustomDataType, IdTokenInfoType, TariffType};
use super::super::enumerations::{AuthorizeCertificateStatusEnumType, EnergyTransferModeEnumType};
use serde::{Deserialize, Serialize};

/// Response to an AuthorizeRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizeResponse {
    /// Contains information about authorization status, expiry and group id.
    pub id_token_info: IdTokenInfoType,

    /// Certificate status information.
    /// - if all certificates are valid: return 'Accepted'.
    /// - if one of the certificates was revoked, return 'CertificateRevoked'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,

    /// List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,

    /// The tariff that is applied to this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff: Option<TariffType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
