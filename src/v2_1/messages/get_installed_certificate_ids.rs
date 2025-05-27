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
