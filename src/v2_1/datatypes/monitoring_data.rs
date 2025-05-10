use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, variable::VariableType,
    variable_monitoring::VariableMonitoringType,
};

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a variable is monitored.
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    pub variable: VariableType,

    /// Required. The type of this monitor, e.g. a threshold, delta or periodic monitor.
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

impl MonitoringDataType {
    /// Creates a new `MonitoringDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a variable is monitored
    /// * `variable` - Variable that is monitored
    /// * `variable_monitoring` - The type of this monitor (threshold, delta or periodic)
    ///
    /// # Returns
    ///
    /// A new instance of `MonitoringDataType` with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        variable_monitoring: Vec<VariableMonitoringType>,
    ) -> Self {
        Self {
            component,
            variable,
            variable_monitoring,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this monitoring data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which a variable is monitored
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a variable is monitored
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
    /// A reference to the variable that is monitored
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable that is monitored
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the variable monitoring types.
    ///
    /// # Returns
    ///
    /// A reference to the vector of variable monitoring types
    pub fn variable_monitoring(&self) -> &Vec<VariableMonitoringType> {
        &self.variable_monitoring
    }

    /// Sets the variable monitoring types.
    ///
    /// # Arguments
    ///
    /// * `variable_monitoring` - The vector of monitoring types (threshold, delta or periodic)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_monitoring(
        &mut self,
        variable_monitoring: Vec<VariableMonitoringType>,
    ) -> &mut Self {
        self.variable_monitoring = variable_monitoring;
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
    /// * `custom_data` - Custom data for this monitoring data, or None to clear
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
    use crate::v2_1::enumerations::MonitorEnumType;

    #[test]
    fn test_new_monitoring_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            MonitorEnumType::UpperThreshold,
            80.0,
        )];

        let monitoring_data = MonitoringDataType::new(
            component.clone(),
            variable.clone(),
            variable_monitoring.clone(),
        );

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            MonitorEnumType::UpperThreshold,
            80.0,
        )];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let monitoring_data = MonitoringDataType::new(
            component.clone(),
            variable.clone(),
            variable_monitoring.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring);
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let variable1 = VariableType::new("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring1 = vec![VariableMonitoringType::new(
            1,
            MonitorEnumType::UpperThreshold,
            80.0,
        )];

        let component2 = ComponentType::new("Meter".to_string());
        let variable2 = VariableType::new("Current".to_string(), "Output".to_string());
        let variable_monitoring2 = vec![
            VariableMonitoringType::new(2, MonitorEnumType::LowerThreshold, 5.0),
            VariableMonitoringType::new(3, MonitorEnumType::UpperThreshold, 32.0),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut monitoring_data =
            MonitoringDataType::new(component1, variable1, variable_monitoring1);

        monitoring_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_variable_monitoring(variable_monitoring2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring_data.component(), &component2);
        assert_eq!(monitoring_data.variable(), &variable2);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring2);
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring_data.set_custom_data(None);
        assert_eq!(monitoring_data.custom_data(), None);
    }
}
