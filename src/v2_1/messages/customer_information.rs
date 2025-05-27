use crate::v2_1::datatypes::{
    CertificateHashDataType, 
    CustomDataType, 
    IdTokenType, 
    StatusInfoType,
};
use crate::v2_1::enumerations::CustomerInformationStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the CustomerInformation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub customer_certificate: Option<CertificateHashDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token: Option<IdTokenType>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    pub report: bool,

    /// Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub clear: bool,

    /// A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub customer_identifier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CustomerInformationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of the request.
    /// * `report` - Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    /// * `clear` - Flag indicating whether the Charging Station should clear all information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, report: bool, clear: bool) -> Self {
        Self {
            customer_certificate: None,
            id_token: None,
            request_id,
            report,
            clear,
            customer_identifier: None,
            custom_data: None,
        }
    }

    /// Sets the customer_certificate field.
    ///
    /// * `customer_certificate` - The customer_certificate field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_customer_certificate(&mut self, customer_certificate: Option<CertificateHashDataType>) -> &mut Self {
        self.customer_certificate = customer_certificate;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the report field.
    ///
    /// * `report` - Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_report(&mut self, report: bool) -> &mut Self {
        self.report = report;
        self
    }

    /// Sets the clear field.
    ///
    /// * `clear` - Flag indicating whether the Charging Station should clear all information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_clear(&mut self, clear: bool) -> &mut Self {
        self.clear = clear;
        self
    }

    /// Sets the customer_identifier field.
    ///
    /// * `customer_identifier` - A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_customer_identifier(&mut self, customer_identifier: Option<String>) -> &mut Self {
        self.customer_identifier = customer_identifier;
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

    /// Gets a reference to the customer_certificate field.
    ///
    /// # Returns
    ///
    /// The customer_certificate field
    pub fn get_customer_certificate(&self) -> Option<&CertificateHashDataType> {
        self.customer_certificate.as_ref()
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the report field.
    ///
    /// # Returns
    ///
    /// Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    pub fn get_report(&self) -> &bool {
        &self.report
    }

    /// Gets a reference to the clear field.
    ///
    /// # Returns
    ///
    /// Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub fn get_clear(&self) -> &bool {
        &self.clear
    }

    /// Gets a reference to the customer_identifier field.
    ///
    /// # Returns
    ///
    /// A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    pub fn get_customer_identifier(&self) -> Option<&String> {
        self.customer_identifier.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the customer_certificate field and returns self for builder pattern.
    ///
    /// * `customer_certificate` - The customer_certificate field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_customer_certificate(mut self, customer_certificate: CertificateHashDataType) -> Self {
        self.customer_certificate = Some(customer_certificate);
        self
    }

    /// Sets the id_token field and returns self for builder pattern.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Sets the customer_identifier field and returns self for builder pattern.
    ///
    /// * `customer_identifier` - A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_customer_identifier(mut self, customer_identifier: String) -> Self {
        self.customer_identifier = Some(customer_identifier);
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

/// Response body for the CustomerInformation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    pub status: CustomerInformationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CustomerInformationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: CustomerInformationStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: CustomerInformationStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &CustomerInformationStatusEnumType {
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
