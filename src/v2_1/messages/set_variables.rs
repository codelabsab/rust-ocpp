use crate::v2_1::datatypes::{CustomDataType, SetVariableDataType, SetVariableResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_variable_data: Vec<SetVariableDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariablesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `set_variable_data` - The set_variable_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_variable_data: Vec<SetVariableDataType>) -> Self {
        Self {
            set_variable_data,
            custom_data: None,
        }
    }

    /// Sets the set_variable_data field.
    ///
    /// * `set_variable_data` - The set_variable_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_variable_data(&mut self, set_variable_data: Vec<SetVariableDataType>) -> &mut Self {
        self.set_variable_data = set_variable_data;
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

    /// Gets a reference to the set_variable_data field.
    ///
    /// # Returns
    ///
    /// The set_variable_data field
    pub fn get_set_variable_data(&self) -> &Vec<SetVariableDataType> {
        &self.set_variable_data
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

/// Response body for the SetVariables response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_variable_result: Vec<SetVariableResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariablesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `set_variable_result` - The set_variable_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_variable_result: Vec<SetVariableResultType>) -> Self {
        Self {
            set_variable_result,
            custom_data: None,
        }
    }

    /// Sets the set_variable_result field.
    ///
    /// * `set_variable_result` - The set_variable_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_variable_result(&mut self, set_variable_result: Vec<SetVariableResultType>) -> &mut Self {
        self.set_variable_result = set_variable_result;
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

    /// Gets a reference to the set_variable_result field.
    ///
    /// # Returns
    ///
    /// The set_variable_result field
    pub fn get_set_variable_result(&self) -> &Vec<SetVariableResultType> {
        &self.set_variable_result
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
