use crate::v2_1::datatypes::{CertificateHashDataType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::DeleteCertificateStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the DeleteCertificate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    #[validate(nested)]
    pub certificate_hash_data: CertificateHashDataType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DeleteCertificateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `certificate_hash_data` - The certificate_hash_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(certificate_hash_data: CertificateHashDataType) -> Self {
        Self {
            certificate_hash_data,
            custom_data: None,
        }
    }

    /// Sets the certificate_hash_data field.
    ///
    /// * `certificate_hash_data` - The certificate_hash_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_hash_data(&mut self, certificate_hash_data: CertificateHashDataType) -> &mut Self {
        self.certificate_hash_data = certificate_hash_data;
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

    /// Gets a reference to the certificate_hash_data field.
    ///
    /// # Returns
    ///
    /// The certificate_hash_data field
    pub fn get_certificate_hash_data(&self) -> &CertificateHashDataType {
        &self.certificate_hash_data
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
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

/// Response body for the DeleteCertificate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    pub status: DeleteCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DeleteCertificateResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DeleteCertificateStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: DeleteCertificateStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &DeleteCertificateStatusEnumType {
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
    use crate::v2_1::enumerations::HashAlgorithmEnumType;
    use serde_json;

    fn create_test_certificate_hash_data() -> CertificateHashDataType {
        CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
        )
    }

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    // Tests for DeleteCertificateRequest

    #[test]
    fn test_delete_certificate_request_new() {
        let cert_hash_data = create_test_certificate_hash_data();
        let request = DeleteCertificateRequest::new(cert_hash_data.clone());

        assert_eq!(request.certificate_hash_data, cert_hash_data);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_delete_certificate_request_with_custom_data() {
        let cert_hash_data = create_test_certificate_hash_data();
        let custom_data = create_test_custom_data();
        let request = DeleteCertificateRequest::new(cert_hash_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.certificate_hash_data, cert_hash_data);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_delete_certificate_request_setters() {
        let cert_hash_data1 = create_test_certificate_hash_data();
        let cert_hash_data2 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "new_issuer_name_hash".to_string(),
            "new_issuer_key_hash".to_string(),
            "new_serial_number".to_string(),
        );
        let custom_data = create_test_custom_data();

        let mut request = DeleteCertificateRequest::new(cert_hash_data1);
        request.set_certificate_hash_data(cert_hash_data2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.certificate_hash_data, cert_hash_data2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_delete_certificate_request_getters() {
        let cert_hash_data = create_test_certificate_hash_data();
        let custom_data = create_test_custom_data();
        let request = DeleteCertificateRequest::new(cert_hash_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_hash_data(), &cert_hash_data);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_delete_certificate_request_serialization() {
        let cert_hash_data = create_test_certificate_hash_data();
        let request = DeleteCertificateRequest::new(cert_hash_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: DeleteCertificateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_delete_certificate_request_deserialization() {
        let json = r#"{
            "certificateHashData": {
                "hashAlgorithm": "SHA256",
                "issuerNameHash": "issuer_name_hash",
                "issuerKeyHash": "issuer_key_hash",
                "serialNumber": "serial_number"
            }
        }"#;

        let request: DeleteCertificateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.certificate_hash_data.hash_algorithm(), &HashAlgorithmEnumType::SHA256);
        assert_eq!(request.certificate_hash_data.issuer_name_hash(), "issuer_name_hash");
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_delete_certificate_request_validation() {
        let cert_hash_data = create_test_certificate_hash_data();
        let request = DeleteCertificateRequest::new(cert_hash_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_delete_certificate_request_json_round_trip() {
        let cert_hash_data = create_test_certificate_hash_data();
        let custom_data = create_test_custom_data();
        let request = DeleteCertificateRequest::new(cert_hash_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: DeleteCertificateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for DeleteCertificateResponse

    #[test]
    fn test_delete_certificate_response_new() {
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Accepted);

        assert_eq!(response.status, DeleteCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_delete_certificate_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Failed)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, DeleteCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_delete_certificate_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::NotFound)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, DeleteCertificateStatusEnumType::NotFound);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_delete_certificate_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Accepted);
        response.set_status(DeleteCertificateStatusEnumType::Failed);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, DeleteCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_delete_certificate_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &DeleteCertificateStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_delete_certificate_response_serialization() {
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: DeleteCertificateResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_delete_certificate_response_deserialization() {
        let json = r#"{
            "status": "Failed",
            "statusInfo": {
                "reasonCode": "CertificateNotFound"
            }
        }"#;

        let response: DeleteCertificateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, DeleteCertificateStatusEnumType::Failed);
        assert!(response.status_info.is_some());
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_delete_certificate_response_validation() {
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_delete_certificate_response_all_status_types() {
        let statuses = vec![
            DeleteCertificateStatusEnumType::Accepted,
            DeleteCertificateStatusEnumType::Failed,
            DeleteCertificateStatusEnumType::NotFound,
        ];

        for status in statuses {
            let response = DeleteCertificateResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_delete_certificate_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = DeleteCertificateResponse::new(DeleteCertificateStatusEnumType::Failed)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: DeleteCertificateResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}
