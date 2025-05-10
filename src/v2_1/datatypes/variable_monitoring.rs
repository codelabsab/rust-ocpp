use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MonitorEnumType;

/// A monitoring setting for a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. ID of the monitor.
    pub id: i32,

    /// Required. Monitor type of the variable.
    pub r#type: MonitorEnumType,

    /// Required. Value for threshold or delta of the monitor.
    pub value: f64,

    /// Optional. The severity that will be assigned to an event that is triggered by this monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
}

impl VariableMonitoringType {
    /// Creates a new `VariableMonitoringType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the monitor
    /// * `type` - Monitor type of the variable
    /// * `value` - Value for threshold or delta of the monitor
    ///
    /// # Returns
    ///
    /// A new `VariableMonitoringType` instance with optional fields set to `None`
    pub fn new(id: i32, r#type: MonitorEnumType, value: f64) -> Self {
        Self {
            custom_data: None,
            id,
            r#type,
            value,
            severity: None,
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
    /// The modified `VariableMonitoringType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the severity field.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn with_severity(mut self, severity: i32) -> Self {
        self.severity = Some(severity);
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
    /// The modified `VariableMonitoringType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the ID of the monitor.
    ///
    /// # Returns
    ///
    /// The ID of the monitor
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID of the monitor.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the monitor type.
    ///
    /// # Returns
    ///
    /// The monitor type of the variable
    pub fn type_field(&self) -> &MonitorEnumType {
        &self.r#type
    }

    /// Sets the monitor type.
    ///
    /// # Arguments
    ///
    /// * `type` - Monitor type of the variable
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_type(&mut self, r#type: MonitorEnumType) -> &mut Self {
        self.r#type = r#type;
        self
    }

    /// Gets the value for threshold or delta.
    ///
    /// # Returns
    ///
    /// The value for threshold or delta of the monitor
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the value for threshold or delta.
    ///
    /// # Arguments
    ///
    /// * `value` - Value for threshold or delta of the monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_value(&mut self, value: f64) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the severity.
    ///
    /// # Returns
    ///
    /// The severity that will be assigned to an event that is triggered by this monitor
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
    /// The modified `VariableMonitoringType` instance
    pub fn set_severity(&mut self, severity: Option<i32>) -> &mut Self {
        self.severity = severity;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_monitoring_new() {
        let id = 42;
        let monitor_type = MonitorEnumType::UpperThreshold;
        let value = 100.0;

        let monitoring = VariableMonitoringType::new(id, monitor_type.clone(), value);

        assert_eq!(monitoring.id(), id);
        assert_eq!(monitoring.type_field(), &monitor_type);
        assert_eq!(monitoring.value(), value);
        assert_eq!(monitoring.severity(), None);
        assert_eq!(monitoring.custom_data(), None);
    }

    #[test]
    fn test_variable_monitoring_with_methods() {
        let id = 42;
        let monitor_type = MonitorEnumType::UpperThreshold;
        let value = 100.0;
        let severity = 2;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let monitoring = VariableMonitoringType::new(id, monitor_type.clone(), value)
            .with_severity(severity)
            .with_custom_data(custom_data.clone());

        assert_eq!(monitoring.id(), id);
        assert_eq!(monitoring.type_field(), &monitor_type);
        assert_eq!(monitoring.value(), value);
        assert_eq!(monitoring.severity(), Some(severity));
        assert_eq!(monitoring.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_variable_monitoring_setters() {
        let id1 = 42;
        let id2 = 43;
        let monitor_type1 = MonitorEnumType::UpperThreshold;
        let monitor_type2 = MonitorEnumType::LowerThreshold;
        let value1 = 100.0;
        let value2 = 50.0;
        let severity = 2;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut monitoring = VariableMonitoringType::new(id1, monitor_type1.clone(), value1);

        monitoring
            .set_id(id2)
            .set_type(monitor_type2.clone())
            .set_value(value2)
            .set_severity(Some(severity))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring.id(), id2);
        assert_eq!(monitoring.type_field(), &monitor_type2);
        assert_eq!(monitoring.value(), value2);
        assert_eq!(monitoring.severity(), Some(severity));
        assert_eq!(monitoring.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring.set_severity(None).set_custom_data(None);

        assert_eq!(monitoring.severity(), None);
        assert_eq!(monitoring.custom_data(), None);
    }
}
