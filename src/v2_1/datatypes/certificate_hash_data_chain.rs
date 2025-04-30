use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::get_certificate_id_use::GetCertificateIdUseEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Certificate hash data chain for validating certificates through OCSP.
///
/// This type represents a chain of certificate hash data used for certificate validation
/// through the Online Certificate Status Protocol (OCSP).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    /// Information to identify a certificate
    pub certificate_hash_data: CertificateHashDataType,

    /// Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType,

    /// Information to identify the child certificate(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateHashDataChainType {
    /// Creates a new `CertificateHashDataChainType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Information to identify a certificate
    /// * `certificate_type` - Type of the requested certificate(s)
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateHashDataChainType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        certificate_type: GetCertificateIdUseEnumType,
    ) -> Self {
        Self {
            certificate_hash_data,
            certificate_type,
            child_certificate_hash_data: None,
            custom_data: None,
        }
    }

    /// Sets the child certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `child_certificate_hash_data` - Information to identify the child certificate(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_child_certificate_hash_data(
        mut self,
        child_certificate_hash_data: Vec<CertificateHashDataType>,
    ) -> Self {
        self.child_certificate_hash_data = Some(child_certificate_hash_data);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate hash data chain
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the certificate hash data.
    ///
    /// # Returns
    ///
    /// A reference to the certificate hash data
    pub fn certificate_hash_data(&self) -> &CertificateHashDataType {
        &self.certificate_hash_data
    }

    /// Sets the certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Information to identify a certificate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_certificate_hash_data(
        &mut self,
        certificate_hash_data: CertificateHashDataType,
    ) -> &mut Self {
        self.certificate_hash_data = certificate_hash_data;
        self
    }

    /// Gets the certificate type.
    ///
    /// # Returns
    ///
    /// The type of the requested certificate(s)
    pub fn certificate_type(&self) -> &GetCertificateIdUseEnumType {
        &self.certificate_type
    }

    /// Sets the certificate type.
    ///
    /// # Arguments
    ///
    /// * `certificate_type` - Type of the requested certificate(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_certificate_type(
        &mut self,
        certificate_type: GetCertificateIdUseEnumType,
    ) -> &mut Self {
        self.certificate_type = certificate_type;
        self
    }

    /// Gets the child certificate hash data.
    ///
    /// # Returns
    ///
    /// An optional reference to the child certificate hash data
    pub fn child_certificate_hash_data(&self) -> Option<&Vec<CertificateHashDataType>> {
        self.child_certificate_hash_data.as_ref()
    }

    /// Sets the child certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `child_certificate_hash_data` - Information to identify the child certificate(s), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_child_certificate_hash_data(
        &mut self,
        child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
    ) -> &mut Self {
        self.child_certificate_hash_data = child_certificate_hash_data;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate hash data chain, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}
