use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::SetVariableStatusEnumType;

/// Class to hold results of SetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which the result is returned.
    pub component: ComponentType,

    /// Required. Variable for which the result is returned.
    pub variable: VariableType,

    /// Optional. Type of attribute for which the result is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub attribute_type: Option<String>,

    /// Required. Result status of setting the variable.
    pub attribute_status: SetVariableStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

impl SetVariableResultType {
    /// Creates a new `SetVariableResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the result is returned
    /// * `variable` - Variable for which the result is returned
    /// * `attribute_status` - Result status of setting the variable
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariableResultType` with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        attribute_status: SetVariableStatusEnumType,
    ) -> Self {
        Self {
            component,
            variable,
            attribute_status,
            custom_data: None,
            attribute_type: None,
            attribute_status_info: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this variable result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Type of attribute for which the result is returned
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_attribute_type(mut self, attribute_type: String) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }

    /// Sets the attribute status info.
    ///
    /// # Arguments
    ///
    /// * `attribute_status_info` - Detailed status information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_attribute_status_info(mut self, attribute_status_info: StatusInfoType) -> Self {
        self.attribute_status_info = Some(attribute_status_info);
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which the result is returned
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the result is returned
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
    /// A reference to the variable for which the result is returned
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the result is returned
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// An optional type of attribute for which the result is returned
    pub fn attribute_type(&self) -> Option<&str> {
        self.attribute_type.as_deref()
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Type of attribute for which the result is returned, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_attribute_type(&mut self, attribute_type: Option<String>) -> &mut Self {
        self.attribute_type = attribute_type;
        self
    }

    /// Gets the attribute status.
    ///
    /// # Returns
    ///
    /// The result status of setting the variable
    pub fn attribute_status(&self) -> SetVariableStatusEnumType {
        self.attribute_status.clone()
    }

    /// Sets the attribute status.
    ///
    /// # Arguments
    ///
    /// * `attribute_status` - Result status of setting the variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_attribute_status(&mut self, attribute_status: SetVariableStatusEnumType) -> &mut Self {
        self.attribute_status = attribute_status;
        self
    }

    /// Gets the attribute status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the detailed status information
    pub fn attribute_status_info(&self) -> Option<&StatusInfoType> {
        self.attribute_status_info.as_ref()
    }

    /// Sets the attribute status info.
    ///
    /// # Arguments
    ///
    /// * `attribute_status_info` - Detailed status information, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_attribute_status_info(&mut self, attribute_status_info: Option<StatusInfoType>) -> &mut Self {
        self.attribute_status_info = attribute_status_info;
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
    /// * `custom_data` - Custom data for this variable result, or None to clear
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
    fn test_new_set_variable_result() {
        let component = ComponentType::new("component1".to_string());
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let attribute_status = SetVariableStatusEnumType::Accepted;

        let result = SetVariableResultType::new(component.clone(), variable.clone(), attribute_status.clone());

        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.attribute_status(), attribute_status);
        assert_eq!(result.attribute_type(), None);
        assert_eq!(result.attribute_status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("component1".to_string());
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let attribute_status = SetVariableStatusEnumType::Accepted;
        let attribute_type = "ActualValue".to_string();
        let attribute_status_info = StatusInfoType {
            reason_code: "NotSupported".to_string(),
            additional_info: None,
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = SetVariableResultType::new(component.clone(), variable.clone(), attribute_status.clone())
            .with_attribute_type(attribute_type.clone())
            .with_attribute_status_info(attribute_status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.attribute_status(), attribute_status);
        assert_eq!(result.attribute_type(), Some(attribute_type.as_str()));
        assert_eq!(result.attribute_status_info(), Some(&attribute_status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("component1".to_string());
        let variable1 = VariableType::new("variable1".to_string(), "instance1".to_string());
        let attribute_status1 = SetVariableStatusEnumType::Accepted;

        let mut result = SetVariableResultType::new(component1, variable1, attribute_status1);

        let component2 = ComponentType::new("component2".to_string());
        let variable2 = VariableType::new("variable2".to_string(), "instance2".to_string());
        let attribute_status2 = SetVariableStatusEnumType::Rejected;
        let attribute_type = "MinValue".to_string();
        let attribute_status_info = StatusInfoType {
            reason_code: "NotSupported".to_string(),
            additional_info: None,
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        result
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_attribute_status(attribute_status2.clone())
            .set_attribute_type(Some(attribute_type.clone()))
            .set_attribute_status_info(Some(attribute_status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.component(), &component2);
        assert_eq!(result.variable(), &variable2);
        assert_eq!(result.attribute_status(), attribute_status2);
        assert_eq!(result.attribute_type(), Some(attribute_type.as_str()));
        assert_eq!(result.attribute_status_info(), Some(&attribute_status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_attribute_type(None)
            .set_attribute_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.attribute_type(), None);
        assert_eq!(result.attribute_status_info(), None);
        assert_eq!(result.custom_data(), None);
    }
}
