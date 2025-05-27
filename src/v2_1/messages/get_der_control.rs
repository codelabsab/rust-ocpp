use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::DERControlEnumType;
use crate::v2_1::enumerations::der_control::DERControlStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlRequest {
    /// RequestId to be used in ReportDERControlRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// True: get a default DER control. False: get a scheduled control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<DERControlEnumType>,

    /// Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub control_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDERControlRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - RequestId to be used in ReportDERControlRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            is_default: None,
            control_type: None,
            control_id: None,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - RequestId to be used in ReportDERControlRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the is_default field.
    ///
    /// * `is_default` - True: get a default DER control. False: get a scheduled control.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_is_default(&mut self, is_default: Option<bool>) -> &mut Self {
        self.is_default = is_default;
        self
    }

    /// Sets the control_type field.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_type(&mut self, control_type: Option<DERControlEnumType>) -> &mut Self {
        self.control_type = control_type;
        self
    }

    /// Sets the control_id field.
    ///
    /// * `control_id` - Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_id(&mut self, control_id: Option<String>) -> &mut Self {
        self.control_id = control_id;
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// RequestId to be used in ReportDERControlRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the is_default field.
    ///
    /// # Returns
    ///
    /// True: get a default DER control. False: get a scheduled control.
    pub fn get_is_default(&self) -> Option<&bool> {
        self.is_default.as_ref()
    }

    /// Gets a reference to the control_type field.
    ///
    /// # Returns
    ///
    /// The control_type field
    pub fn get_control_type(&self) -> Option<&DERControlEnumType> {
        self.control_type.as_ref()
    }

    /// Gets a reference to the control_id field.
    ///
    /// # Returns
    ///
    /// Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    pub fn get_control_id(&self) -> Option<&String> {
        self.control_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the is_default field and returns self for builder pattern.
    ///
    /// * `is_default` - True: get a default DER control. False: get a scheduled control.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    /// Sets the control_type field and returns self for builder pattern.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_control_type(mut self, control_type: DERControlEnumType) -> Self {
        self.control_type = Some(control_type);
        self
    }

    /// Sets the control_id field and returns self for builder pattern.
    ///
    /// * `control_id` - Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_control_id(mut self, control_id: String) -> Self {
        self.control_id = Some(control_id);
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

/// Response body for the GetDERControl response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlResponse {
    pub status: DERControlStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDERControlResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DERControlStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: DERControlStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &DERControlStatusEnumType {
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
