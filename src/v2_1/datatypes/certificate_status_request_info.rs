use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::CertificateStatusSourceEnumType;
use serde::{Deserialize, Serialize};

/// Data necessary to request the revocation status of a certificate.
///
/// This type contains the information needed to request the revocation status
/// of a certificate from a certificate status source like OCSP or CRL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusRequestInfoType {
    /// Certificate hash data needed for validating certificates through OCSP.
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// URL(s) of _source_.
    pub urls: Vec<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateStatusRequestInfoType {
    /// Creates a new `CertificateStatusRequestInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Certificate hash data for validation
    /// * `source` - Source of the certificate status information
    /// * `urls` - URLs of the certificate status source
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateStatusRequestInfoType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        source: CertificateStatusSourceEnumType,
        urls: Vec<String>,
    ) -> Self {
        Self {
            certificate_hash_data,
            source,
            urls,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate status request
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
    /// * `certificate_hash_data` - Certificate hash data for validation
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

    /// Gets the source of the certificate status.
    ///
    /// # Returns
    ///
    /// The source of the certificate status information
    pub fn source(&self) -> &CertificateStatusSourceEnumType {
        &self.source
    }

    /// Sets the source of the certificate status.
    ///
    /// # Arguments
    ///
    /// * `source` - Source of the certificate status information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_source(&mut self, source: CertificateStatusSourceEnumType) -> &mut Self {
        self.source = source;
        self
    }

    /// Gets the URLs of the certificate status source.
    ///
    /// # Returns
    ///
    /// A reference to the URLs of the certificate status source
    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }

    /// Sets the URLs of the certificate status source.
    ///
    /// # Arguments
    ///
    /// * `urls` - URLs of the certificate status source
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_urls(&mut self, urls: Vec<String>) -> &mut Self {
        self.urls = urls;
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
    /// * `custom_data` - Custom data for this certificate status request, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::HashAlgorithmEnumType;

    #[test]
    fn test_new_certificate_status_request_info() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let urls = vec!["https://ocsp.example.com".to_string()];

        let request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls.clone(),
        );

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data);
        assert_eq!(request_info.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(request_info.urls(), &urls);
        assert_eq!(request_info.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let urls = vec!["https://ocsp.example.com".to_string()];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data);
        assert_eq!(request_info.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(request_info.urls(), &urls);
        assert_eq!(request_info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let cert_hash_data1 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let cert_hash_data2 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "new_name_hash".to_string(),
            "new_key_hash".to_string(),
            "new_serial".to_string(),
        );

        let urls1 = vec!["https://ocsp.example.com".to_string()];
        let urls2 = vec!["https://crl.example.com".to_string(), "https://crl2.example.com".to_string()];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data1.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls1.clone(),
        );

        request_info
            .set_certificate_hash_data(cert_hash_data2.clone())
            .set_source(CertificateStatusSourceEnumType::CRL)
            .set_urls(urls2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data2);
        assert_eq!(request_info.source(), &CertificateStatusSourceEnumType::CRL);
        assert_eq!(request_info.urls(), &urls2);
        assert_eq!(request_info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        request_info.set_custom_data(None);
        assert_eq!(request_info.custom_data(), None);
    }
}
