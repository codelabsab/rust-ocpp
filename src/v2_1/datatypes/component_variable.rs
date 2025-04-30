use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,

    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}

impl ComponentVariableType {
    /// Creates a new `ComponentVariableType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    ///
    /// # Returns
    ///
    /// A new instance of `ComponentVariableType` with optional fields set to `None`
    pub fn new(component: ComponentType) -> Self {
        Self {
            component,
            custom_data: None,
            variable: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this component variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the report is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_variable(mut self, variable: VariableType) -> Self {
        self.variable = Some(variable);
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
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
    /// * `custom_data` - Custom data for this component variable, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// An optional reference to the variable
    pub fn variable(&self) -> Option<&VariableType> {
        self.variable.as_ref()
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the report is requested, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: Option<VariableType>) -> &mut Self {
        self.variable = variable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_component_variable() {
        let component = ComponentType::new("Connector".to_string());
        let component_variable = ComponentVariableType::new(component.clone());

        assert_eq!(component_variable.component(), &component);
        assert_eq!(component_variable.custom_data(), None);
        assert_eq!(component_variable.variable(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string());

        let component_variable = ComponentVariableType::new(component.clone())
            .with_custom_data(custom_data.clone())
            .with_variable(variable.clone());

        assert_eq!(component_variable.component(), &component);
        assert_eq!(component_variable.custom_data(), Some(&custom_data));
        assert_eq!(component_variable.variable(), Some(&variable));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let component2 = ComponentType::new("Meter".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable = VariableType::new("CurrentLimit".to_string(), "Secondary".to_string());

        let mut component_variable = ComponentVariableType::new(component1);

        component_variable
            .set_component(component2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_variable(Some(variable.clone()));

        assert_eq!(component_variable.component(), &component2);
        assert_eq!(component_variable.custom_data(), Some(&custom_data));
        assert_eq!(component_variable.variable(), Some(&variable));

        // Test clearing optional fields
        component_variable.set_custom_data(None).set_variable(None);

        assert_eq!(component_variable.custom_data(), None);
        assert_eq!(component_variable.variable(), None);
    }
}
