use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataElementType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Variable for which the stream data is reported.
    pub variable: VariableType,

    /// Required. The value for the variable.
    #[validate(length(max = 2500))]
    pub value: String,

    /// Required. Sequence number for stream data.
    #[validate(range(min = 0))]
    pub sequence_id: i32,
}

impl StreamDataElementType {
    /// Creates a new `StreamDataElementType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the stream data is reported
    /// * `value` - The value for the variable
    /// * `sequence_id` - Sequence number for stream data
    ///
    /// # Returns
    ///
    /// A new instance of `StreamDataElementType` with optional fields set to `None`
    pub fn new(variable: VariableType, value: String, sequence_id: i32) -> Self {
        Self {
            custom_data: None,
            variable,
            value,
            sequence_id,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this stream data element
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable for which the stream data is reported
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the stream data is reported
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the value.
    ///
    /// # Returns
    ///
    /// The value for the variable
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value for the variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the sequence ID.
    ///
    /// # Returns
    ///
    /// The sequence number for stream data
    pub fn sequence_id(&self) -> i32 {
        self.sequence_id
    }

    /// Sets the sequence ID.
    ///
    /// # Arguments
    ///
    /// * `sequence_id` - Sequence number for stream data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sequence_id(&mut self, sequence_id: i32) -> &mut Self {
        self.sequence_id = sequence_id;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this stream data element, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stream_data_element() {
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let value = "42.5".to_string();
        let sequence_id = 1;

        let element = StreamDataElementType::new(variable.clone(), value.clone(), sequence_id);

        assert_eq!(element.variable(), &variable);
        assert_eq!(element.value(), value);
        assert_eq!(element.sequence_id(), sequence_id);
        assert_eq!(element.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let value = "42.5".to_string();
        let sequence_id = 1;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let element = StreamDataElementType::new(variable.clone(), value.clone(), sequence_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(element.variable(), &variable);
        assert_eq!(element.value(), value);
        assert_eq!(element.sequence_id(), sequence_id);
        assert_eq!(element.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let variable1 = VariableType::new("variable1".to_string(), "instance1".to_string());
        let value1 = "42.5".to_string();
        let sequence_id1 = 1;

        let mut element = StreamDataElementType::new(variable1, value1, sequence_id1);

        let variable2 = VariableType::new("variable2".to_string(), "instance2".to_string());
        let value2 = "84.0".to_string();
        let sequence_id2 = 2;
        let custom_data = CustomDataType::new("VendorX".to_string());

        element
            .set_variable(variable2.clone())
            .set_value(value2.clone())
            .set_sequence_id(sequence_id2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(element.variable(), &variable2);
        assert_eq!(element.value(), value2);
        assert_eq!(element.sequence_id(), sequence_id2);
        assert_eq!(element.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        element.set_custom_data(None);

        assert_eq!(element.custom_data(), None);
    }
}
