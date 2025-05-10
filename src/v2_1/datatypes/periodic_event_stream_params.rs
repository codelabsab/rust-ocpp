use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for periodic event stream configuration.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicEventStreamParamsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The interval in seconds after which the Charging Station shall send the event information.
    #[validate(range(min = 1, max = 86400))]
    pub reporting_interval: i32,
}

impl PeriodicEventStreamParamsType {
    /// Creates a new `PeriodicEventStreamParamsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `reporting_interval` - The interval in seconds after which the Charging Station shall send the event information
    ///
    /// # Returns
    ///
    /// A new instance of `PeriodicEventStreamParamsType` with optional fields set to `None`
    pub fn new(reporting_interval: i32) -> Self {
        Self {
            reporting_interval,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the reporting interval.
    ///
    /// # Returns
    ///
    /// The interval in seconds after which the Charging Station shall send the event information
    pub fn reporting_interval(&self) -> i32 {
        self.reporting_interval
    }

    /// Sets the reporting interval.
    ///
    /// # Arguments
    ///
    /// * `reporting_interval` - The interval in seconds after which the Charging Station shall send the event information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reporting_interval(&mut self, reporting_interval: i32) -> &mut Self {
        self.reporting_interval = reporting_interval;
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
    /// * `custom_data` - Custom data for these parameters, or None to clear
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
    fn test_new_periodic_event_stream_params() {
        let params = PeriodicEventStreamParamsType::new(60);

        assert_eq!(params.reporting_interval(), 60);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = PeriodicEventStreamParamsType::new(60).with_custom_data(custom_data.clone());

        assert_eq!(params.reporting_interval(), 60);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = PeriodicEventStreamParamsType::new(60);

        params
            .set_reporting_interval(120)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.reporting_interval(), 120);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }
}
