use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::GetVariableStatusEnumType;

/// Class to hold results of GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which the Variable is requested.
    pub component: ComponentType,

    /// Required. Variable for which the attribute value is requested.
    pub variable: VariableType,

    /// Optional. If the variable is attribute-based, this field specifies the attribute type for which the value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub attribute_type: Option<String>,

    /// Optional. Value of the requested attribute if status is Accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2500))]
    pub attribute_value: Option<String>,

    /// Required. Result status of getting the variable.
    pub attribute_status: GetVariableStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

impl GetVariableResultType {
    /// Creates a new `GetVariableResultType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the Variable is requested
    /// * `variable` - Variable for which the attribute value is requested
    /// * `attribute_status` - Result status of getting the variable
    ///
    /// # Returns
    ///
    /// A new `GetVariableResultType` instance with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        attribute_status: GetVariableStatusEnumType,
    ) -> Self {
        Self {
            custom_data: None,
            component,
            variable,
            attribute_type: None,
            attribute_value: None,
            attribute_status,
            attribute_status_info: None,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the attribute type field.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - The attribute type for which the value is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_type(mut self, attribute_type: String) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }

    /// Sets the attribute value field.
    ///
    /// # Arguments
    ///
    /// * `attribute_value` - Value of the requested attribute
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_value(mut self, attribute_value: String) -> Self {
        self.attribute_value = Some(attribute_value);
        self
    }

    /// Sets the attribute status info field.
    ///
    /// # Arguments
    ///
    /// * `attribute_status_info` - Detailed status information
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_status_info(mut self, attribute_status_info: StatusInfoType) -> Self {
        self.attribute_status_info = Some(attribute_status_info);
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
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
    /// * `component` - Component for which the Variable is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
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
    /// * `variable` - Variable for which the attribute value is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute type
    pub fn attribute_type(&self) -> Option<&str> {
        self.attribute_type.as_deref()
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - The attribute type for which the value is requested, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_type(&mut self, attribute_type: Option<String>) -> &mut Self {
        self.attribute_type = attribute_type;
        self
    }

    /// Gets the attribute value.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute value
    pub fn attribute_value(&self) -> Option<&str> {
        self.attribute_value.as_deref()
    }

    /// Sets the attribute value.
    ///
    /// # Arguments
    ///
    /// * `attribute_value` - Value of the requested attribute, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_value(&mut self, attribute_value: Option<String>) -> &mut Self {
        self.attribute_value = attribute_value;
        self
    }

    /// Gets the attribute status.
    ///
    /// # Returns
    ///
    /// The attribute status
    pub fn attribute_status(&self) -> &GetVariableStatusEnumType {
        &self.attribute_status
    }

    /// Sets the attribute status.
    ///
    /// # Arguments
    ///
    /// * `attribute_status` - Result status of getting the variable
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_status(
        &mut self,
        attribute_status: GetVariableStatusEnumType,
    ) -> &mut Self {
        self.attribute_status = attribute_status;
        self
    }

    /// Gets the attribute status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute status info
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
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_status_info(
        &mut self,
        attribute_status_info: Option<StatusInfoType>,
    ) -> &mut Self {
        self.attribute_status_info = attribute_status_info;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_variable_result_new() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let attribute_status = GetVariableStatusEnumType::Accepted;

        let result = GetVariableResultType::new(
            component.clone(),
            variable.clone(),
            attribute_status.clone(),
        );

        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.attribute_status(), &attribute_status);
        assert_eq!(result.attribute_type(), None);
        assert_eq!(result.attribute_value(), None);
        assert_eq!(result.attribute_status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_get_variable_result_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let attribute_status = GetVariableStatusEnumType::Accepted;
        let attribute_type = "Actual".to_string();
        let attribute_value = "25.5".to_string();
        let attribute_status_info = StatusInfoType::new("Additional info".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = GetVariableResultType::new(
            component.clone(),
            variable.clone(),
            attribute_status.clone(),
        )
        .with_attribute_type(attribute_type.clone())
        .with_attribute_value(attribute_value.clone())
        .with_attribute_status_info(attribute_status_info.clone())
        .with_custom_data(custom_data.clone());

        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.attribute_status(), &attribute_status);
        assert_eq!(result.attribute_type(), Some(attribute_type.as_str()));
        assert_eq!(result.attribute_value(), Some(attribute_value.as_str()));
        assert_eq!(result.attribute_status_info(), Some(&attribute_status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_variable_result_setters() {
        let component1 = ComponentType::new("Connector".to_string());
        let component2 = ComponentType::new("Meter".to_string());
        let variable1 = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let variable2 = VariableType::new("Current".to_string(), "Output".to_string());
        let attribute_status1 = GetVariableStatusEnumType::Accepted;
        let attribute_status2 = GetVariableStatusEnumType::Rejected;
        let attribute_type = "Actual".to_string();
        let attribute_value = "25.5".to_string();
        let attribute_status_info = StatusInfoType::new("Additional info".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut result = GetVariableResultType::new(
            component1.clone(),
            variable1.clone(),
            attribute_status1.clone(),
        );

        result
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_attribute_status(attribute_status2.clone())
            .set_attribute_type(Some(attribute_type.clone()))
            .set_attribute_value(Some(attribute_value.clone()))
            .set_attribute_status_info(Some(attribute_status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.component(), &component2);
        assert_eq!(result.variable(), &variable2);
        assert_eq!(result.attribute_status(), &attribute_status2);
        assert_eq!(result.attribute_type(), Some(attribute_type.as_str()));
        assert_eq!(result.attribute_value(), Some(attribute_value.as_str()));
        assert_eq!(result.attribute_status_info(), Some(&attribute_status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_attribute_type(None)
            .set_attribute_value(None)
            .set_attribute_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.attribute_type(), None);
        assert_eq!(result.attribute_value(), None);
        assert_eq!(result.attribute_status_info(), None);
        assert_eq!(result.custom_data(), None);
    }
}
