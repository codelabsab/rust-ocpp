use crate::v2_1::datatypes::{CertificateHashDataChainType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{GetCertificateIdUseEnumType, GetInstalledCertificateStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetInstalledCertificateIds request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    /// Indicates the type of certificates requested. When omitted, all certificate types are requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetInstalledCertificateIdsRequest {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            certificate_type: None,
            custom_data: None,
        }
    }

    /// Sets the certificate_type field.
    ///
    /// * `certificate_type` - Indicates the type of certificates requested. When omitted, all certificate types are requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_type(&mut self, certificate_type: Option<Vec<GetCertificateIdUseEnumType>>) -> &mut Self {
        self.certificate_type = certificate_type;
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

    /// Gets a reference to the certificate_type field.
    ///
    /// # Returns
    ///
    /// Indicates the type of certificates requested. When omitted, all certificate types are requested.
    pub fn get_certificate_type(&self) -> Option<&Vec<GetCertificateIdUseEnumType>> {
        self.certificate_type.as_ref()
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
    /// * `certificate_type` - Indicates the type of certificates requested. When omitted, all certificate types are requested.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_certificate_type(mut self, certificate_type: Vec<GetCertificateIdUseEnumType>) -> Self {
        self.certificate_type = Some(certificate_type);
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

/// Response body for the GetInstalledCertificateIds response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    pub status: GetInstalledCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetInstalledCertificateIdsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GetInstalledCertificateStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            certificate_hash_data_chain: None,
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
    pub fn set_status(&mut self, status: GetInstalledCertificateStatusEnumType) -> &mut Self {
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

    /// Sets the certificate_hash_data_chain field.
    ///
    /// * `certificate_hash_data_chain` - The certificate_hash_data_chain field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_hash_data_chain(&mut self, certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>) -> &mut Self {
        self.certificate_hash_data_chain = certificate_hash_data_chain;
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
    pub fn get_status(&self) -> &GetInstalledCertificateStatusEnumType {
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

    /// Gets a reference to the certificate_hash_data_chain field.
    ///
    /// # Returns
    ///
    /// The certificate_hash_data_chain field
    pub fn get_certificate_hash_data_chain(&self) -> Option<&Vec<CertificateHashDataChainType>> {
        self.certificate_hash_data_chain.as_ref()
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

    /// Sets the certificate_hash_data_chain field and returns self for builder pattern.
    ///
    /// * `certificate_hash_data_chain` - The certificate_hash_data_chain field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_certificate_hash_data_chain(mut self, certificate_hash_data_chain: Vec<CertificateHashDataChainType>) -> Self {
        self.certificate_hash_data_chain = Some(certificate_hash_data_chain);
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
    use crate::v2_1::datatypes::CertificateHashDataType;
    use crate::v2_1::enumerations::HashAlgorithmEnumType;
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_certificate_hash_data() -> CertificateHashDataType {
        CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
        )
    }

    fn create_test_certificate_hash_data_chain() -> CertificateHashDataChainType {
        CertificateHashDataChainType::new(
            create_test_certificate_hash_data(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        )
    }

    // Tests for GetInstalledCertificateIdsRequest

    #[test]
    fn test_get_installed_certificate_ids_request_new() {
        let request = GetInstalledCertificateIdsRequest::new();

        assert_eq!(request.certificate_type, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_installed_certificate_ids_request_with_certificate_type() {
        let cert_types = vec![
            GetCertificateIdUseEnumType::CSMSRootCertificate,
            GetCertificateIdUseEnumType::V2GRootCertificate,
        ];
        let request = GetInstalledCertificateIdsRequest::new()
            .with_certificate_type(cert_types.clone());

        assert_eq!(request.certificate_type, Some(cert_types));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_installed_certificate_ids_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetInstalledCertificateIdsRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.certificate_type, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_request_setters() {
        let cert_types = vec![GetCertificateIdUseEnumType::MORootCertificate];
        let custom_data = create_test_custom_data();

        let mut request = GetInstalledCertificateIdsRequest::new();
        request.set_certificate_type(Some(cert_types.clone()));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.certificate_type, Some(cert_types));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_request_getters() {
        let cert_types = vec![GetCertificateIdUseEnumType::V2GCertificateChain];
        let custom_data = create_test_custom_data();
        let request = GetInstalledCertificateIdsRequest::new()
            .with_certificate_type(cert_types.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_type(), Some(&cert_types));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_request_serialization() {
        let request = GetInstalledCertificateIdsRequest::new();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetInstalledCertificateIdsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_installed_certificate_ids_request_validation() {
        let request = GetInstalledCertificateIdsRequest::new();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_installed_certificate_ids_request_validation_empty_certificate_type() {
        let mut request = GetInstalledCertificateIdsRequest::new();
        request.set_certificate_type(Some(vec![])); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_installed_certificate_ids_request_all_certificate_types() {
        let cert_types = vec![
            GetCertificateIdUseEnumType::V2GRootCertificate,
            GetCertificateIdUseEnumType::MORootCertificate,
            GetCertificateIdUseEnumType::CSMSRootCertificate,
            GetCertificateIdUseEnumType::V2GCertificateChain,
            GetCertificateIdUseEnumType::ManufacturerRootCertificate,
            GetCertificateIdUseEnumType::OEMRootCertificate,
        ];

        for cert_type in cert_types {
            let request = GetInstalledCertificateIdsRequest::new()
                .with_certificate_type(vec![cert_type.clone()]);
            assert_eq!(request.certificate_type, Some(vec![cert_type]));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_installed_certificate_ids_request_json_round_trip() {
        let cert_types = vec![
            GetCertificateIdUseEnumType::CSMSRootCertificate,
            GetCertificateIdUseEnumType::V2GRootCertificate,
        ];
        let custom_data = create_test_custom_data();
        let request = GetInstalledCertificateIdsRequest::new()
            .with_certificate_type(cert_types)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetInstalledCertificateIdsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetInstalledCertificateIdsResponse

    #[test]
    fn test_get_installed_certificate_ids_response_new() {
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted);

        assert_eq!(response.status, GetInstalledCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.certificate_hash_data_chain, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_installed_certificate_ids_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::NotFound)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GetInstalledCertificateStatusEnumType::NotFound);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.certificate_hash_data_chain, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_installed_certificate_ids_response_with_certificate_hash_data_chain() {
        let cert_chain = vec![create_test_certificate_hash_data_chain()];
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted)
            .with_certificate_hash_data_chain(cert_chain.clone());

        assert_eq!(response.status, GetInstalledCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.certificate_hash_data_chain, Some(cert_chain));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_installed_certificate_ids_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GetInstalledCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.certificate_hash_data_chain, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_response_setters() {
        let status_info = create_test_status_info();
        let cert_chain = vec![create_test_certificate_hash_data_chain()];
        let custom_data = create_test_custom_data();

        let mut response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted);
        response.set_status(GetInstalledCertificateStatusEnumType::NotFound);
        response.set_status_info(Some(status_info.clone()));
        response.set_certificate_hash_data_chain(Some(cert_chain.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GetInstalledCertificateStatusEnumType::NotFound);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.certificate_hash_data_chain, Some(cert_chain));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_response_getters() {
        let status_info = create_test_status_info();
        let cert_chain = vec![create_test_certificate_hash_data_chain()];
        let custom_data = create_test_custom_data();
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_certificate_hash_data_chain(cert_chain.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GetInstalledCertificateStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_certificate_hash_data_chain(), Some(&cert_chain));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_installed_certificate_ids_response_serialization() {
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetInstalledCertificateIdsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_installed_certificate_ids_response_validation() {
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_installed_certificate_ids_response_validation_empty_certificate_chain() {
        let mut response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted);
        response.set_certificate_hash_data_chain(Some(vec![])); // Empty list should fail validation

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_installed_certificate_ids_response_all_status_types() {
        let statuses = vec![
            GetInstalledCertificateStatusEnumType::Accepted,
            GetInstalledCertificateStatusEnumType::NotFound,
        ];

        for status in statuses {
            let response = GetInstalledCertificateIdsResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_installed_certificate_ids_response_multiple_certificate_chains() {
        let cert_chain1 = CertificateHashDataChainType::new(
            create_test_certificate_hash_data(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        );
        let cert_chain2 = CertificateHashDataChainType::new(
            create_test_certificate_hash_data(),
            GetCertificateIdUseEnumType::V2GRootCertificate,
        );
        let cert_chains = vec![cert_chain1, cert_chain2];

        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted)
            .with_certificate_hash_data_chain(cert_chains.clone());

        assert_eq!(response.certificate_hash_data_chain, Some(cert_chains));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_installed_certificate_ids_response_json_round_trip() {
        let status_info = create_test_status_info();
        let cert_chain = vec![create_test_certificate_hash_data_chain()];
        let custom_data = create_test_custom_data();
        let response = GetInstalledCertificateIdsResponse::new(GetInstalledCertificateStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_certificate_hash_data_chain(cert_chain)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetInstalledCertificateIdsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}