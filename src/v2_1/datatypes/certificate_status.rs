use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::{CertificateStatusEnumType, CertificateStatusSourceEnumType};
use serde::{Deserialize, Serialize};

/// Revocation status of certificate
///
/// This type represents the status of a certificate, including its revocation status,
/// source of the status information, and when the next update is expected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusType {
    /// Certificate hash data needed for validating certificates through OCSP.
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// Status of certificate: good, revoked or unknown.
    pub status: CertificateStatusEnumType,

    /// The date and time at which the next update of the certificate status MAY be expected.
    pub next_update: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateStatusType {
    /// Creates a new `CertificateStatusType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Certificate hash data for validation
    /// * `source` - Source of the certificate status information
    /// * `status` - Status of the certificate
    /// * `next_update` - Expected time of next status update
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateStatusType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        source: CertificateStatusSourceEnumType,
        status: CertificateStatusEnumType,
        next_update: String,
    ) -> Self {
        Self {
            certificate_hash_data,
            source,
            status,
            next_update,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate status
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

    /// Gets the status of the certificate.
    ///
    /// # Returns
    ///
    /// The status of the certificate
    pub fn status(&self) -> &CertificateStatusEnumType {
        &self.status
    }

    /// Sets the status of the certificate.
    ///
    /// # Arguments
    ///
    /// * `status` - Status of the certificate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: CertificateStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the next update time.
    ///
    /// # Returns
    ///
    /// The expected time of the next status update
    pub fn next_update(&self) -> &str {
        &self.next_update
    }

    /// Sets the next update time.
    ///
    /// # Arguments
    ///
    /// * `next_update` - Expected time of next status update
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_next_update(&mut self, next_update: String) -> &mut Self {
        self.next_update = next_update;
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
    /// * `custom_data` - Custom data for this certificate status, or None to clear
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
    fn test_new_certificate_status() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let cert_status = CertificateStatusType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            "2023-01-01T00:00:00Z".to_string(),
        );

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Good);
        assert_eq!(cert_status.next_update(), "2023-01-01T00:00:00Z");
        assert_eq!(cert_status.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string());

        let cert_status = CertificateStatusType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            "2023-01-01T00:00:00Z".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Good);
        assert_eq!(cert_status.next_update(), "2023-01-01T00:00:00Z");
        assert_eq!(cert_status.custom_data(), Some(&custom_data));
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

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut cert_status = CertificateStatusType::new(
            cert_hash_data1.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            "2023-01-01T00:00:00Z".to_string(),
        );

        cert_status
            .set_certificate_hash_data(cert_hash_data2.clone())
            .set_source(CertificateStatusSourceEnumType::CRL)
            .set_status(CertificateStatusEnumType::Revoked)
            .set_next_update("2023-02-01T00:00:00Z".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data2);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::CRL);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Revoked);
        assert_eq!(cert_status.next_update(), "2023-02-01T00:00:00Z");
        assert_eq!(cert_status.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cert_status.set_custom_data(None);
        assert_eq!(cert_status.custom_data(), None);
    }
}
