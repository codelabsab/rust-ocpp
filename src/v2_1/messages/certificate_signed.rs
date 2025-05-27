use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{CertificateSignedStatusEnumType, CertificateSigningUseEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the CertificateSigned request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    /// The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate.  The Configuration Variable &lt;&lt;configkey-max-certificate-chain-size,MaxCertificateChainSize&gt;&gt; can be used to limit the maximum size of this field.
    #[validate(length(max = 10000))]
    pub certificate_chain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    /// *(2.1)* RequestId to correlate this message with the SignCertificateRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub request_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateSignedRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `certificate_chain` - The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate.  The Configuration Variable &lt;&lt;configkey-max-certificate-chain-size,MaxCertificateChainSize&gt;&gt; can be used to limit the maximum size of this field.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(certificate_chain: String) -> Self {
        Self {
            certificate_chain,
            certificate_type: None,
            request_id: None,
            custom_data: None,
        }
    }

    /// Sets the certificate_chain field.
    ///
    /// * `certificate_chain` - The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate.  The Configuration Variable &lt;&lt;configkey-max-certificate-chain-size,MaxCertificateChainSize&gt;&gt; can be used to limit the maximum size of this field.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_chain(&mut self, certificate_chain: String) -> &mut Self {
        self.certificate_chain = certificate_chain;
        self
    }

    /// Sets the certificate_type field.
    ///
    /// * `certificate_type` - The certificate_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_type(&mut self, certificate_type: Option<CertificateSigningUseEnumType>) -> &mut Self {
        self.certificate_type = certificate_type;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - *(2.1)* RequestId to correlate this message with the SignCertificateRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: Option<i32>) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the certificate_chain field.
    ///
    /// # Returns
    ///
    /// The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate.  The Configuration Variable &lt;&lt;configkey-max-certificate-chain-size,MaxCertificateChainSize&gt;&gt; can be used to limit the maximum size of this field.
    pub fn get_certificate_chain(&self) -> &String {
        &self.certificate_chain
    }

    /// Gets a reference to the certificate_type field.
    ///
    /// # Returns
    ///
    /// The certificate_type field
    pub fn get_certificate_type(&self) -> Option<&CertificateSigningUseEnumType> {
        self.certificate_type.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// *(2.1)* RequestId to correlate this message with the SignCertificateRequest.
    pub fn get_request_id(&self) -> Option<&i32> {
        self.request_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the certificate_type field and returns self for builder pattern.
    ///
    /// * `certificate_type` - The certificate_type field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_certificate_type(mut self, certificate_type: CertificateSigningUseEnumType) -> Self {
        self.certificate_type = Some(certificate_type);
        self
    }

    /// Sets the request_id field and returns self for builder pattern.
    ///
    /// * `request_id` - *(2.1)* RequestId to correlate this message with the SignCertificateRequest.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_request_id(mut self, request_id: i32) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the CertificateSigned response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    pub status: CertificateSignedStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateSignedResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: CertificateSignedStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: CertificateSignedStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &CertificateSignedStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
