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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use validator::Validate;

    fn create_test_certificate_chain() -> String {
        "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKHHH4HH4HH4MA0GCSqGSIb3DQEBCwUAMBQxEjAQBgNVBAMMCVRl\nc3QgQ2VydDAeFw0yMzEyMjUxMDMwMDBaFw0yNDEyMjQxMDMwMDBaMBQxEjAQBgNV\nBAMMCVRlc3QgQ2VydDBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABH4H4H4H4H4H\n4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H\n4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H\n4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H4H\n-----END CERTIFICATE-----".to_string()
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_certificate_signed_request_new() {
        let certificate_chain = create_test_certificate_chain();
        let request = CertificateSignedRequest::new(certificate_chain.clone());

        assert_eq!(request.get_certificate_chain(), &certificate_chain);
        assert_eq!(request.get_certificate_type(), None);
        assert_eq!(request.get_request_id(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_certificate_signed_request_validation() {
        let certificate_chain = create_test_certificate_chain();
        let request = CertificateSignedRequest::new(certificate_chain);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_request_validation_too_long_chain() {
        let certificate_chain = "x".repeat(10001); // Invalid: max 10000 chars
        let request = CertificateSignedRequest::new(certificate_chain);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_certificate_signed_request_validation_invalid_request_id() {
        let certificate_chain = create_test_certificate_chain();
        let mut request = CertificateSignedRequest::new(certificate_chain);
        request.set_request_id(Some(-1)); // Invalid: must be >= 0

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_certificate_signed_request_serialization() {
        let certificate_chain = create_test_certificate_chain();
        let request = CertificateSignedRequest::new(certificate_chain);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_certificate_signed_request_with_certificate_type() {
        let certificate_chain = create_test_certificate_chain();
        let certificate_type = CertificateSigningUseEnumType::ChargingStationCertificate;

        let request = CertificateSignedRequest::new(certificate_chain)
            .with_certificate_type(certificate_type.clone());

        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
    }

    #[test]
    fn test_certificate_signed_request_with_request_id() {
        let certificate_chain = create_test_certificate_chain();
        let request_id = 123;

        let request = CertificateSignedRequest::new(certificate_chain)
            .with_request_id(request_id);

        assert_eq!(request.get_request_id(), Some(&request_id));
    }

    #[test]
    fn test_certificate_signed_request_with_custom_data() {
        let certificate_chain = create_test_certificate_chain();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CertificateSignedRequest::new(certificate_chain)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_certificate_signed_request_set_methods() {
        let certificate_chain = create_test_certificate_chain();
        let new_certificate_chain = "-----BEGIN CERTIFICATE-----\nNEW_CERT_DATA\n-----END CERTIFICATE-----".to_string();
        let certificate_type = CertificateSigningUseEnumType::V2GCertificate;
        let request_id = 456;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CertificateSignedRequest::new(certificate_chain);

        request
            .set_certificate_chain(new_certificate_chain.clone())
            .set_certificate_type(Some(certificate_type.clone()))
            .set_request_id(Some(request_id))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_certificate_chain(), &new_certificate_chain);
        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
        assert_eq!(request.get_request_id(), Some(&request_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_certificate_signed_request_all_certificate_types() {
        let certificate_chain = create_test_certificate_chain();

        let certificate_types = vec![
            CertificateSigningUseEnumType::ChargingStationCertificate,
            CertificateSigningUseEnumType::V2GCertificate,
            CertificateSigningUseEnumType::V2G20Certificate,
        ];

        for certificate_type in certificate_types {
            let request = CertificateSignedRequest::new(certificate_chain.clone())
                .with_certificate_type(certificate_type.clone());
            assert_eq!(request.get_certificate_type(), Some(&certificate_type));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_certificate_signed_request_edge_cases() {
        let certificate_chain = create_test_certificate_chain();

        // Test with minimum valid request_id
        let request = CertificateSignedRequest::new(certificate_chain.clone())
            .with_request_id(0);
        assert_eq!(request.get_request_id(), Some(&0));
        assert!(request.validate().is_ok());

        // Test with large request_id
        let request = CertificateSignedRequest::new(certificate_chain.clone())
            .with_request_id(i32::MAX);
        assert_eq!(request.get_request_id(), Some(&i32::MAX));
        assert!(request.validate().is_ok());

        // Test with maximum length certificate chain
        let max_chain = "x".repeat(10000);
        let request = CertificateSignedRequest::new(max_chain.clone());
        assert_eq!(request.get_certificate_chain(), &max_chain);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_response_new() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let response = CertificateSignedResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_certificate_signed_response_validation() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let response = CertificateSignedResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_response_serialization() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let response = CertificateSignedResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CertificateSignedResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_certificate_signed_response_with_status_info() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = CertificateSignedResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_certificate_signed_response_with_custom_data() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CertificateSignedResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_certificate_signed_response_set_methods() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let new_status = CertificateSignedStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = CertificateSignedResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_certificate_signed_response_all_status_values() {
        let status_values = vec![
            CertificateSignedStatusEnumType::Accepted,
            CertificateSignedStatusEnumType::Rejected,
        ];

        for status in status_values {
            let response = CertificateSignedResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_certificate_signed_response_builder_pattern() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CertificateSignedResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_certificate_signed_request_json_round_trip() {
        let certificate_chain = create_test_certificate_chain();
        let certificate_type = CertificateSigningUseEnumType::V2GCertificate;
        let request_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CertificateSignedRequest::new(certificate_chain)
            .with_certificate_type(certificate_type)
            .with_request_id(request_id)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_response_json_round_trip() {
        let status = CertificateSignedStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CertificateSignedResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CertificateSignedResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_response_with_detailed_status_info() {
        let status = CertificateSignedStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("InvalidCertificate".to_string())
            .with_additional_info("The provided certificate chain is invalid or corrupted".to_string());

        let response = CertificateSignedResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CertificateSignedResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_certificate_signed_request_clear_optional_fields() {
        let certificate_chain = create_test_certificate_chain();
        let certificate_type = CertificateSigningUseEnumType::ChargingStationCertificate;
        let request_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CertificateSignedRequest::new(certificate_chain)
            .with_certificate_type(certificate_type)
            .with_request_id(request_id)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(request.get_certificate_type().is_some());
        assert!(request.get_request_id().is_some());
        assert!(request.get_custom_data().is_some());

        // Clear optional fields
        request.set_certificate_type(None);
        request.set_request_id(None);
        request.set_custom_data(None);

        assert_eq!(request.get_certificate_type(), None);
        assert_eq!(request.get_request_id(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_certificate_signed_response_clear_optional_fields() {
        let status = CertificateSignedStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = CertificateSignedResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(response.get_status_info().is_some());
        assert!(response.get_custom_data().is_some());

        // Clear optional fields
        response.set_status_info(None);
        response.set_custom_data(None);

        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_certificate_signed_request_with_complex_custom_data() {
        use serde_json::json;

        let certificate_chain = create_test_certificate_chain();
        let custom_data = CustomDataType::new("CertificateVendor".to_string())
            .with_property("issuer".to_string(), json!("Test CA"))
            .with_property("validity".to_string(), json!("365 days"))
            .with_property("metadata".to_string(), json!({
                "algorithm": "RSA-2048",
                "purpose": "charging_station_auth"
            }));

        let request = CertificateSignedRequest::new(certificate_chain)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_certificate_signed_request_certificate_chain_formats() {
        // Test with different certificate chain formats
        let chains = vec![
            // Single certificate
            "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKHHH4HH4HH4MA0GCSqGSIb3DQEBCwUAMBQxEjAQBgNVBAMMCVRl\n-----END CERTIFICATE-----".to_string(),

            // Certificate chain with multiple certificates
            "-----BEGIN CERTIFICATE-----\nLEAF_CERT_DATA\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nINTERMEDIATE_CERT_DATA\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nROOT_CERT_DATA\n-----END CERTIFICATE-----".to_string(),

            // Minimal valid certificate
            "-----BEGIN CERTIFICATE-----\nDATA\n-----END CERTIFICATE-----".to_string(),
        ];

        for chain in chains {
            let request = CertificateSignedRequest::new(chain.clone());
            assert_eq!(request.get_certificate_chain(), &chain);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_certificate_signed_response_status_semantics() {
        // Test Accepted status - certificate was successfully processed
        let accepted_response = CertificateSignedResponse::new(CertificateSignedStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string())
                .with_additional_info("Certificate installed successfully".to_string()));

        assert_eq!(accepted_response.get_status(), &CertificateSignedStatusEnumType::Accepted);
        assert!(accepted_response.validate().is_ok());

        // Test Rejected status - certificate was rejected
        let rejected_response = CertificateSignedResponse::new(CertificateSignedStatusEnumType::Rejected)
            .with_status_info(StatusInfoType::new("InvalidCertificate".to_string())
                .with_additional_info("Certificate validation failed".to_string()));

        assert_eq!(rejected_response.get_status(), &CertificateSignedStatusEnumType::Rejected);
        assert!(rejected_response.validate().is_ok());
    }

    #[test]
    fn test_certificate_signed_request_minimal_valid() {
        // Test minimal valid request (only required fields)
        let certificate_chain = create_test_certificate_chain();
        let request = CertificateSignedRequest::new(certificate_chain.clone());

        assert_eq!(request.get_certificate_chain(), &certificate_chain);
        assert_eq!(request.get_certificate_type(), None);
        assert_eq!(request.get_request_id(), None);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());

        // Test serialization of minimal request
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_certificate_signed_response_minimal_valid() {
        // Test minimal valid response (only required fields)
        let response = CertificateSignedResponse::new(CertificateSignedStatusEnumType::Accepted);

        assert_eq!(response.get_status(), &CertificateSignedStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());

        // Test serialization of minimal response
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CertificateSignedResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_certificate_signed_request_full_configuration() {
        // Test request with all optional fields set
        let certificate_chain = create_test_certificate_chain();
        let certificate_type = CertificateSigningUseEnumType::V2G20Certificate;
        let request_id = 789;
        let custom_data = CustomDataType::new("FullConfigVendor".to_string());

        let request = CertificateSignedRequest::new(certificate_chain.clone())
            .with_certificate_type(certificate_type.clone())
            .with_request_id(request_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_chain(), &certificate_chain);
        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
        assert_eq!(request.get_request_id(), Some(&request_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());

        // Test serialization with all fields
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }
}