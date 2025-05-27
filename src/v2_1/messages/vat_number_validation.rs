use crate::v2_1::datatypes::{AddressType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the VatNumberValidation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationRequest {
    /// VAT number to check.
    #[validate(length(max = 20))]
    pub vat_number: String,

    /// EVSE id for which check is done
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VatNumberValidationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `vat_number` - VAT number to check.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(vat_number: String) -> Self {
        Self {
            vat_number,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the vat_number field.
    ///
    /// * `vat_number` - VAT number to check.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_vat_number(&mut self, vat_number: String) -> &mut Self {
        self.vat_number = vat_number;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id for which check is done
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
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

    /// Gets a reference to the vat_number field.
    ///
    /// # Returns
    ///
    /// VAT number to check.
    pub fn get_vat_number(&self) -> &String {
        &self.vat_number
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id for which check is done
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - EVSE id for which check is done
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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

/// Response body for the VatNumberValidation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub company: Option<AddressType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// VAT number that was requested.
    #[validate(length(max = 20))]
    pub vat_number: String,

    /// EVSE id for which check was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VatNumberValidationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `vat_number` - VAT number that was requested.
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(vat_number: String, status: GenericStatusEnumType) -> Self {
        Self {
            company: None,
            status_info: None,
            vat_number,
            evse_id: None,
            status,
            custom_data: None,
        }
    }

    /// Sets the company field.
    ///
    /// * `company` - The company field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_company(&mut self, company: Option<AddressType>) -> &mut Self {
        self.company = company;
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

    /// Sets the vat_number field.
    ///
    /// * `vat_number` - VAT number that was requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_vat_number(&mut self, vat_number: String) -> &mut Self {
        self.vat_number = vat_number;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id for which check was requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
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

    /// Gets a reference to the company field.
    ///
    /// # Returns
    ///
    /// The company field
    pub fn get_company(&self) -> Option<&AddressType> {
        self.company.as_ref()
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the vat_number field.
    ///
    /// # Returns
    ///
    /// VAT number that was requested.
    pub fn get_vat_number(&self) -> &String {
        &self.vat_number
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id for which check was requested.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &GenericStatusEnumType {
        &self.status
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the company field and returns self for builder pattern.
    ///
    /// * `company` - The company field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_company(mut self, company: AddressType) -> Self {
        self.company = Some(company);
        self
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

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - EVSE id for which check was requested.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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
