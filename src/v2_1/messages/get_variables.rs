use crate::v2_1::datatypes::{CustomDataType, GetVariableDataType, GetVariableResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub get_variable_data: Vec<GetVariableDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariablesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `get_variable_data` - The get_variable_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(get_variable_data: Vec<GetVariableDataType>) -> Self {
        Self {
            get_variable_data,
            custom_data: None,
        }
    }

    /// Sets the get_variable_data field.
    ///
    /// * `get_variable_data` - The get_variable_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_get_variable_data(&mut self, get_variable_data: Vec<GetVariableDataType>) -> &mut Self {
        self.get_variable_data = get_variable_data;
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

    /// Gets a reference to the get_variable_data field.
    ///
    /// # Returns
    ///
    /// The get_variable_data field
    pub fn get_get_variable_data(&self) -> &Vec<GetVariableDataType> {
        &self.get_variable_data
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

/// Response body for the GetVariables response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub get_variable_result: Vec<GetVariableResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariablesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `get_variable_result` - The get_variable_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(get_variable_result: Vec<GetVariableResultType>) -> Self {
        Self {
            get_variable_result,
            custom_data: None,
        }
    }

    /// Sets the get_variable_result field.
    ///
    /// * `get_variable_result` - The get_variable_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_get_variable_result(&mut self, get_variable_result: Vec<GetVariableResultType>) -> &mut Self {
        self.get_variable_result = get_variable_result;
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

    /// Gets a reference to the get_variable_result field.
    ///
    /// # Returns
    ///
    /// The get_variable_result field
    pub fn get_get_variable_result(&self) -> &Vec<GetVariableResultType> {
        &self.get_variable_result
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
