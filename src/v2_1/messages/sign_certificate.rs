use crate::v2_1::datatypes::{CertificateHashDataType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{CertificateSigningUseEnumType, GenericStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SignCertificate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    /// The Charging Station SHALL send the public key in form of a Certificate Signing Request (CSR) as described in RFC 2986 [22] and then PEM encoded, using the &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; message.
    #[validate(length(max = 5500))]
    pub csr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub hash_root_certificate: Option<CertificateHashDataType>,

    /// *(2.1)* RequestId to match this message with the CertificateSignedRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub request_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SignCertificateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `csr` - The Charging Station SHALL send the public key in form of a Certificate Signing Request (CSR) as described in RFC 2986 [22] and then PEM encoded, using the &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; message.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(csr: String) -> Self {
        Self {
            csr,
            certificate_type: None,
            hash_root_certificate: None,
            request_id: None,
            custom_data: None,
        }
    }

    /// Sets the csr field.
    ///
    /// * `csr` - The Charging Station SHALL send the public key in form of a Certificate Signing Request (CSR) as described in RFC 2986 [22] and then PEM encoded, using the &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; message.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_csr(&mut self, csr: String) -> &mut Self {
        self.csr = csr;
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

    /// Sets the hash_root_certificate field.
    ///
    /// * `hash_root_certificate` - The hash_root_certificate field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_hash_root_certificate(&mut self, hash_root_certificate: Option<CertificateHashDataType>) -> &mut Self {
        self.hash_root_certificate = hash_root_certificate;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - *(2.1)* RequestId to match this message with the CertificateSignedRequest.
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

    /// Gets a reference to the csr field.
    ///
    /// # Returns
    ///
    /// The Charging Station SHALL send the public key in form of a Certificate Signing Request (CSR) as described in RFC 2986 [22] and then PEM encoded, using the &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; message.
    pub fn get_csr(&self) -> &String {
        &self.csr
    }

    /// Gets a reference to the certificate_type field.
    ///
    /// # Returns
    ///
    /// The certificate_type field
    pub fn get_certificate_type(&self) -> Option<&CertificateSigningUseEnumType> {
        self.certificate_type.as_ref()
    }

    /// Gets a reference to the hash_root_certificate field.
    ///
    /// # Returns
    ///
    /// The hash_root_certificate field
    pub fn get_hash_root_certificate(&self) -> Option<&CertificateHashDataType> {
        self.hash_root_certificate.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// *(2.1)* RequestId to match this message with the CertificateSignedRequest.
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

    /// Sets the hash_root_certificate field and returns self for builder pattern.
    ///
    /// * `hash_root_certificate` - The hash_root_certificate field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_hash_root_certificate(mut self, hash_root_certificate: CertificateHashDataType) -> Self {
        self.hash_root_certificate = Some(hash_root_certificate);
        self
    }

    /// Sets the request_id field and returns self for builder pattern.
    ///
    /// * `request_id` - *(2.1)* RequestId to match this message with the CertificateSignedRequest.
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

/// Response body for the SignCertificate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SignCertificateResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GenericStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GenericStatusEnumType {
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
    use crate::v2_1::datatypes::{CertificateHashDataType, CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::{CertificateSigningUseEnumType, GenericStatusEnumType, HashAlgorithmEnumType};
    use serde_json;
    use validator::Validate;

    fn create_test_certificate_hash_data() -> CertificateHashDataType {
        CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "abcd1234".to_string(),
            "efgh5678".to_string(),
            "1234567890".to_string(),
        )
    }

    // Tests for SignCertificateRequest

    #[test]
    fn test_sign_certificate_request_new() {
        let csr = "-----BEGIN CERTIFICATE REQUEST-----\nMIICZjCCAU4CAQAwGTEXMBUGA1UEAwwOQ2hhcmdpbmdTdGF0aW9u\n-----END CERTIFICATE REQUEST-----".to_string();
        let request = SignCertificateRequest::new(csr.clone());

        assert_eq!(request.get_csr(), &csr);
        assert_eq!(request.get_certificate_type(), None);
        assert_eq!(request.get_hash_root_certificate(), None);
        assert_eq!(request.get_request_id(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_sign_certificate_request_serialization() {
        let csr = "test_csr".to_string();
        let request = SignCertificateRequest::new(csr);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SignCertificateRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_sign_certificate_request_validation() {
        let csr = "test_csr".to_string();
        let request = SignCertificateRequest::new(csr);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_sign_certificate_request_with_certificate_type() {
        let csr = "test_csr".to_string();
        let certificate_type = CertificateSigningUseEnumType::ChargingStationCertificate;
        let request = SignCertificateRequest::new(csr)
            .with_certificate_type(certificate_type.clone());

        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
    }

    #[test]
    fn test_sign_certificate_request_with_hash_root_certificate() {
        let csr = "test_csr".to_string();
        let hash_data = create_test_certificate_hash_data();
        let request = SignCertificateRequest::new(csr)
            .with_hash_root_certificate(hash_data.clone());

        assert_eq!(request.get_hash_root_certificate(), Some(&hash_data));
    }

    #[test]
    fn test_sign_certificate_request_with_request_id() {
        let csr = "test_csr".to_string();
        let request_id = 123;
        let request = SignCertificateRequest::new(csr)
            .with_request_id(request_id);

        assert_eq!(request.get_request_id(), Some(&request_id));
    }

    #[test]
    fn test_sign_certificate_request_with_custom_data() {
        let csr = "test_csr".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SignCertificateRequest::new(csr)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_sign_certificate_request_set_methods() {
        let csr = "test_csr".to_string();
        let new_csr = "new_test_csr".to_string();
        let certificate_type = CertificateSigningUseEnumType::V2GCertificate;
        let hash_data = create_test_certificate_hash_data();
        let request_id = 456;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = SignCertificateRequest::new(csr);

        request
            .set_csr(new_csr.clone())
            .set_certificate_type(Some(certificate_type.clone()))
            .set_hash_root_certificate(Some(hash_data.clone()))
            .set_request_id(Some(request_id))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_csr(), &new_csr);
        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
        assert_eq!(request.get_hash_root_certificate(), Some(&hash_data));
        assert_eq!(request.get_request_id(), Some(&request_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_sign_certificate_request_builder_pattern() {
        let csr = "test_csr".to_string();
        let certificate_type = CertificateSigningUseEnumType::V2G20Certificate;
        let hash_data = create_test_certificate_hash_data();
        let request_id = 789;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = SignCertificateRequest::new(csr)
            .with_certificate_type(certificate_type.clone())
            .with_hash_root_certificate(hash_data.clone())
            .with_request_id(request_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_type(), Some(&certificate_type));
        assert_eq!(request.get_hash_root_certificate(), Some(&hash_data));
        assert_eq!(request.get_request_id(), Some(&request_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    // Tests for SignCertificateResponse

    #[test]
    fn test_sign_certificate_response_new() {
        let status = GenericStatusEnumType::Accepted;
        let response = SignCertificateResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_sign_certificate_response_serialization() {
        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SignCertificateResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_sign_certificate_response_validation() {
        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_sign_certificate_response_with_status_info() {
        let status_info = StatusInfoType::new("Accepted".to_string());
        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_sign_certificate_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SignCertificateResponse::new(GenericStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_sign_certificate_response_set_methods() {
        let status_info = StatusInfoType::new("Rejected".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = SignCertificateResponse::new(GenericStatusEnumType::Accepted);

        response
            .set_status(GenericStatusEnumType::Rejected)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &GenericStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_sign_certificate_response_builder_pattern() {
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_sign_certificate_request_json_round_trip() {
        let csr = "test_csr_data".to_string();
        let certificate_type = CertificateSigningUseEnumType::ChargingStationCertificate;
        let hash_data = create_test_certificate_hash_data();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SignCertificateRequest::new(csr)
            .with_certificate_type(certificate_type)
            .with_hash_root_certificate(hash_data)
            .with_request_id(123)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SignCertificateRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_sign_certificate_response_json_round_trip() {
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SignCertificateResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_sign_certificate_all_certificate_types() {
        let certificate_types = vec![
            CertificateSigningUseEnumType::ChargingStationCertificate,
            CertificateSigningUseEnumType::V2GCertificate,
            CertificateSigningUseEnumType::V2G20Certificate,
        ];

        for cert_type in certificate_types {
            let request = SignCertificateRequest::new("test_csr".to_string())
                .with_certificate_type(cert_type.clone());
            
            assert_eq!(request.get_certificate_type(), Some(&cert_type));
            assert!(request.validate().is_ok());

            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: SignCertificateRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_sign_certificate_all_status_types() {
        let status_types = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in status_types {
            let response = SignCertificateResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: SignCertificateResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_sign_certificate_request_csr_validation() {
        let long_csr = "x".repeat(5500); // Max allowed length
        let request = SignCertificateRequest::new(long_csr);

        assert!(request.validate().is_ok());

        let too_long_csr = "x".repeat(5501); // Exceeds max length
        let invalid_request = SignCertificateRequest::new(too_long_csr);

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_sign_certificate_request_negative_request_id_validation() {
        let request = SignCertificateRequest::new("test_csr".to_string())
            .with_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_sign_certificate_request_zero_request_id_validation() {
        let request = SignCertificateRequest::new("test_csr".to_string())
            .with_request_id(0);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_sign_certificate_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SignCertificateResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GenericStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }
}