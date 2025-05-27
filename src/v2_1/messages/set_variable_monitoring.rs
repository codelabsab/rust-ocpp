use crate::v2_1::datatypes::{CustomDataType, SetMonitoringDataType, SetMonitoringResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_monitoring_data: Vec<SetMonitoringDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariableMonitoringRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `set_monitoring_data` - The set_monitoring_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_monitoring_data: Vec<SetMonitoringDataType>) -> Self {
        Self {
            set_monitoring_data,
            custom_data: None,
        }
    }

    /// Sets the set_monitoring_data field.
    ///
    /// * `set_monitoring_data` - The set_monitoring_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_monitoring_data(&mut self, set_monitoring_data: Vec<SetMonitoringDataType>) -> &mut Self {
        self.set_monitoring_data = set_monitoring_data;
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

    /// Gets a reference to the set_monitoring_data field.
    ///
    /// # Returns
    ///
    /// The set_monitoring_data field
    pub fn get_set_monitoring_data(&self) -> &Vec<SetMonitoringDataType> {
        &self.set_monitoring_data
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

/// Response body for the SetVariableMonitoring response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariableMonitoringResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `set_monitoring_result` - The set_monitoring_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_monitoring_result: Vec<SetMonitoringResultType>) -> Self {
        Self {
            set_monitoring_result,
            custom_data: None,
        }
    }

    /// Sets the set_monitoring_result field.
    ///
    /// * `set_monitoring_result` - The set_monitoring_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_monitoring_result(&mut self, set_monitoring_result: Vec<SetMonitoringResultType>) -> &mut Self {
        self.set_monitoring_result = set_monitoring_result;
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

    /// Gets a reference to the set_monitoring_result field.
    ///
    /// # Returns
    ///
    /// The set_monitoring_result field
    pub fn get_set_monitoring_result(&self) -> &Vec<SetMonitoringResultType> {
        &self.set_monitoring_result
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
