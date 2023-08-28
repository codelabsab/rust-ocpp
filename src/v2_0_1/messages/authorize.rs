//! Authorize
use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
#[cfg(feature = "std")]
use validator::Validate;

use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::ocsp_request_data_type::OCSPRequestDataType;
use crate::v2_0_1::enumerations::authorize_certificate_status_enum_type::AuthorizeCertificateStatusEnumType;
use crate::Vec;

/// ´AuthorizeRequest`, sent by the Charging Station to the CSMS.
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest<'a, const N_TOKEN_ADDITIONAL_INFOS: usize, const N_ISO_15158_CERTIFICATES: usize> {
    /// The X.509 certificated presented by EV and encoded in PEM format.
    #[cfg_attr(feature="std", validate(length(min = 0, max = 5500)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<&'a str>,
    /// This contains the identifier that needs to be authorized.
    pub id_token: IdTokenType<'a, N_TOKEN_ADDITIONAL_INFOS>,
    /// Contains the information needed to verify the EV Contract Certificate via OCSP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType<'a>, N_ISO_15158_CERTIFICATES>>,
}

/// `AuthorizeResponse`, sent by the
/// CSMS to the Charging Station in response to an [`AuthorizeRequest`].
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse<'a, const N_EVSE_IDS: usize, const TOKEN_N_ADDITIONAL_INFOS: usize> {
    /// Certificate status information. - if all certificates are valid: return
    /// `Accepted`. - if one of the certificates was revoked, return `CertificateRevoked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
    /// Contains information about authorization status, expiry and group id.
    #[serde(borrow)]
    pub id_token_info: IdTokenInfoType<'a, N_EVSE_IDS, TOKEN_N_ADDITIONAL_INFOS>,
}
