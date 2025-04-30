use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, variable::VariableType,
    variable_monitoring::VariableMonitoringType,
};

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a variable is monitored.
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    pub variable: VariableType,

    /// Required. The type of this monitor, e.g. a threshold, delta or periodic monitor.
    #[validate(length(min = 1))]
    pub variable_monitoring: Vec<VariableMonitoringType>,

    /// Optional. The severity that will be assigned to an event that is triggered by this monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
}

impl SetMonitoringDataType {
    /// Creates a new `SetMonitoringDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a variable is monitored
    /// * `variable` - Variable that is monitored
    /// * `variable_monitoring` - The type of this monitor, e.g. a threshold, delta or periodic monitor
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringDataType` with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        variable_monitoring: Vec<VariableMonitoringType>,
    ) -> Self {
        Self {
            custom_data: None,
            component,
            variable,
            variable_monitoring,
            severity: None,
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

    /// Sets the severity.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_severity(mut self, severity: i32) -> Self {
        self.severity = Some(severity);
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

    /// Gets the variable monitoring.
    ///
    /// # Returns
    ///
    /// A reference to the type of this monitor
    pub fn variable_monitoring(&self) -> &[VariableMonitoringType] {
        &self.variable_monitoring
    }

    /// Sets the variable monitoring.
    ///
    /// # Arguments
    ///
    /// * `variable_monitoring` - The type of this monitor, e.g. a threshold, delta or periodic monitor
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

    /// Gets the severity.
    ///
    /// # Returns
    ///
    /// An optional severity that will be assigned to an event that is triggered by this monitor
    pub fn severity(&self) -> Option<i32> {
        self.severity
    }

    /// Sets the severity.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_severity(&mut self, severity: Option<i32>) -> &mut Self {
        self.severity = severity;
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

    #[test]
    fn test_new_monitoring_data() {
        let component = ComponentType::new("component1".to_string());
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let monitoring = VariableMonitoringType::new(
            1,
            crate::v2_1::enumerations::MonitorEnumType::UpperThreshold,
            100.0,
        );
        let monitoring_data = SetMonitoringDataType::new(
            component.clone(),
            variable.clone(),
            vec![monitoring.clone()],
        );

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring().len(), 1);
        assert_eq!(monitoring_data.variable_monitoring()[0], monitoring);
        assert_eq!(monitoring_data.severity(), None);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("component1".to_string());
        let variable = VariableType::new("variable1".to_string(), "instance1".to_string());
        let monitoring = VariableMonitoringType::new(
            1,
            crate::v2_1::enumerations::MonitorEnumType::UpperThreshold,
            100.0,
        );
        let custom_data = CustomDataType::new("VendorX".to_string());
        let severity = 2;

        let monitoring_data = SetMonitoringDataType::new(
            component.clone(),
            variable.clone(),
            vec![monitoring.clone()],
        )
        .with_custom_data(custom_data.clone())
        .with_severity(severity);

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring().len(), 1);
        assert_eq!(monitoring_data.variable_monitoring()[0], monitoring);
        assert_eq!(monitoring_data.severity(), Some(severity));
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("component1".to_string());
        let variable1 = VariableType::new("variable1".to_string(), "instance1".to_string());
        let monitoring1 = VariableMonitoringType::new(
            1,
            crate::v2_1::enumerations::MonitorEnumType::UpperThreshold,
            100.0,
        );

        let mut monitoring_data =
            SetMonitoringDataType::new(component1, variable1, vec![monitoring1]);

        let component2 = ComponentType::new("component2".to_string());
        let variable2 = VariableType::new("variable2".to_string(), "instance2".to_string());
        let monitoring2 = VariableMonitoringType::new(
            2,
            crate::v2_1::enumerations::MonitorEnumType::LowerThreshold,
            50.0,
        );
        let custom_data = CustomDataType::new("VendorX".to_string());
        let severity = 2;

        monitoring_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_variable_monitoring(vec![monitoring2.clone()])
            .set_severity(Some(severity))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring_data.component(), &component2);
        assert_eq!(monitoring_data.variable(), &variable2);
        assert_eq!(monitoring_data.variable_monitoring().len(), 1);
        assert_eq!(monitoring_data.variable_monitoring()[0], monitoring2);
        assert_eq!(monitoring_data.severity(), Some(severity));
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring_data.set_severity(None).set_custom_data(None);

        assert_eq!(monitoring_data.severity(), None);
        assert_eq!(monitoring_data.custom_data(), None);
    }
}
