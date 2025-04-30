use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,

    /// Required. Variable for which a report is requested.
    pub variable: VariableType,

    /// Optional. The actual value of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2500))]
    pub variable_value: Option<String>,

    /// Optional. The attribute type for which a report of variable attribute value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub variable_attribute: Option<String>,
}

impl ReportDataType {
    /// Creates a new `ReportDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    /// * `variable` - Variable for which a report is requested
    ///
    /// # Returns
    ///
    /// A new instance of `ReportDataType` with optional fields set to `None`
    pub fn new(component: ComponentType, variable: VariableType) -> Self {
        Self {
            component,
            variable,
            custom_data: None,
            variable_value: None,
            variable_attribute: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this report data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the variable value.
    ///
    /// # Arguments
    ///
    /// * `variable_value` - The actual value of the variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_variable_value(mut self, variable_value: String) -> Self {
        self.variable_value = Some(variable_value);
        self
    }

    /// Sets the variable attribute.
    ///
    /// # Arguments
    ///
    /// * `variable_attribute` - The attribute type for which a report of variable attribute value is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_variable_attribute(mut self, variable_attribute: String) -> Self {
        self.variable_attribute = Some(variable_attribute);
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

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which a report is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the variable value.
    ///
    /// # Returns
    ///
    /// An optional reference to the variable value
    pub fn variable_value(&self) -> Option<&str> {
        self.variable_value.as_deref()
    }

    /// Sets the variable value.
    ///
    /// # Arguments
    ///
    /// * `variable_value` - The actual value of the variable, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_value(&mut self, variable_value: Option<String>) -> &mut Self {
        self.variable_value = variable_value;
        self
    }

    /// Gets the variable attribute.
    ///
    /// # Returns
    ///
    /// An optional reference to the variable attribute
    pub fn variable_attribute(&self) -> Option<&str> {
        self.variable_attribute.as_deref()
    }

    /// Sets the variable attribute.
    ///
    /// # Arguments
    ///
    /// * `variable_attribute` - The attribute type for which a report of variable attribute value is requested, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_attribute(&mut self, variable_attribute: Option<String>) -> &mut Self {
        self.variable_attribute = variable_attribute;
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
    /// * `custom_data` - Custom data for this report data, or None to clear
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
    fn test_new_report_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string());

        let report_data = ReportDataType::new(component.clone(), variable.clone());

        assert_eq!(report_data.component(), &component);
        assert_eq!(report_data.variable(), &variable);
        assert_eq!(report_data.custom_data(), None);
        assert_eq!(report_data.variable_value(), None);
        assert_eq!(report_data.variable_attribute(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("CurrentLimit".to_string(), "Main".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable_value = "100".to_string();
        let variable_attribute = "MaxValue".to_string();

        let report_data = ReportDataType::new(component.clone(), variable.clone())
            .with_custom_data(custom_data.clone())
            .with_variable_value(variable_value.clone())
            .with_variable_attribute(variable_attribute.clone());

        assert_eq!(report_data.component(), &component);
        assert_eq!(report_data.variable(), &variable);
        assert_eq!(report_data.custom_data(), Some(&custom_data));
        assert_eq!(report_data.variable_value(), Some(variable_value.as_str()));
        assert_eq!(
            report_data.variable_attribute(),
            Some(variable_attribute.as_str())
        );
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let variable1 = VariableType::new("CurrentLimit".to_string(), "Main".to_string());
        let component2 = ComponentType::new("Meter".to_string());
        let variable2 = VariableType::new("VoltageLimit".to_string(), "Secondary".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable_value = "100".to_string();
        let variable_attribute = "MaxValue".to_string();

        let mut report_data = ReportDataType::new(component1, variable1);

        report_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_variable_value(Some(variable_value.clone()))
            .set_variable_attribute(Some(variable_attribute.clone()));

        assert_eq!(report_data.component(), &component2);
        assert_eq!(report_data.variable(), &variable2);
        assert_eq!(report_data.custom_data(), Some(&custom_data));
        assert_eq!(report_data.variable_value(), Some(variable_value.as_str()));
        assert_eq!(
            report_data.variable_attribute(),
            Some(variable_attribute.as_str())
        );

        // Test clearing optional fields
        report_data
            .set_custom_data(None)
            .set_variable_value(None)
            .set_variable_attribute(None);

        assert_eq!(report_data.custom_data(), None);
        assert_eq!(report_data.variable_value(), None);
        assert_eq!(report_data.variable_attribute(), None);
    }
}
