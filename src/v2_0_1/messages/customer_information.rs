//! CustomerInformation
#[cfg(feature = "std")]
use validator::Validate;

use crate::v2_0_1::datatypes::certificate_hash_data_type::CertificateHashDataType;
use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::customer_information_status_enum_type::CustomerInformationStatusEnumType;

/// CustomerInformationRequest, sent by the CSMS to the Charging Station
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest<'a, const TOKEN_N_ADDITIONAL_INFOS: usize> {
    /// The Id of the request
    pub request_id: i64,
    /// Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest
    ///  messages containing information about the customer referred to.
    pub report: bool,
    /// Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub clear: bool,
    /// A (e.g. vendor specific) identifier of the customer this request refers to.
    /// This field contains a custom identifier other than IdToken and Certificate.
    ///  One of the possible identifiers (customerIdentifier, customerIdToken or
    /// customerCertificate) should be in the request message.
    #[cfg_attr(feature="std", validate(length(min = 0, max = 64)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<&'a str>,
    /// The IdToken of the customer this request refers to. One of the possible identifiers
    /// (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType<'a, TOKEN_N_ADDITIONAL_INFOS>>,
    /// The Certificate of the customer this request refers to. One of the possible identifiers
    /// (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<CertificateHashDataType<'a>>,
}

/// CustomerInformationResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse<'a> {
    /// Indicates whether the request was accepted.
    pub status: CustomerInformationStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
