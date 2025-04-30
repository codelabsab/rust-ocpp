use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Reference key to a component-variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Name of the variable. Name should be taken from the list of standardized variable names whenever possible.
    #[validate(length(max = 50))]
    pub name: String,

    /// Required. Name of instance in case the variable exists as multiple instances.
    #[validate(length(max = 50))]
    pub instance: String,
}

impl VariableType {
    /// Creates a new `VariableType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    /// * `instance` - Name of instance in case the variable exists as multiple instances
    ///
    /// # Returns
    ///
    /// A new instance of `VariableType` with optional fields set to `None`
    pub fn new(name: String, instance: String) -> Self {
        Self {
            name,
            instance,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the name.
    ///
    /// # Returns
    ///
    /// A reference to the name of the variable
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Gets the instance.
    ///
    /// # Returns
    ///
    /// A reference to the instance name
    pub fn instance(&self) -> &str {
        &self.instance
    }

    /// Sets the instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Instance name for this variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_instance(&mut self, instance: String) -> &mut Self {
        self.instance = instance;
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
    /// * `custom_data` - Custom data for this variable, or None to clear
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
    fn test_new_variable() {
        let variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string());

        assert_eq!(variable.name(), "CurrentLimit");
        assert_eq!(variable.instance(), "Main");
        assert_eq!(variable.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(variable.name(), "CurrentLimit");
        assert_eq!(variable.instance(), "Main");
        assert_eq!(variable.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string());

        variable
            .set_name("VoltageLimit".to_string())
            .set_instance("Secondary".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(variable.name(), "VoltageLimit");
        assert_eq!(variable.instance(), "Secondary");
        assert_eq!(variable.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        variable.set_custom_data(None);
        assert_eq!(variable.custom_data(), None);
    }
}
