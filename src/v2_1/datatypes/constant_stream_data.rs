use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, periodic_event_stream_params::PeriodicEventStreamParamsType,
};

/// Constant stream data type for periodic event streams.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConstantStreamDataType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Uniquely identifies the stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Parameters for the periodic event stream.
    pub params: PeriodicEventStreamParamsType,

    /// Id of monitor used to report this event. It can be a preconfigured or hardwired monitor.
    #[validate(range(min = 0))]
    pub variable_monitoring_id: i32,
}

impl ConstantStreamDataType {
    /// Creates a new `ConstantStreamDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Uniquely identifies the stream
    /// * `params` - Parameters for the periodic event stream
    /// * `variable_monitoring_id` - Id of monitor used to report this event
    ///
    /// # Returns
    ///
    /// A new instance of `ConstantStreamDataType` with optional fields set to `None`
    pub fn new(
        id: i32,
        params: PeriodicEventStreamParamsType,
        variable_monitoring_id: i32,
    ) -> Self {
        Self {
            id,
            params,
            variable_monitoring_id,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this constant stream data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the stream ID.
    ///
    /// # Returns
    ///
    /// The unique identifier of the stream
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the stream ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Uniquely identifies the stream
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters for the periodic event stream
    pub fn params(&self) -> &PeriodicEventStreamParamsType {
        &self.params
    }

    /// Sets the parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for the periodic event stream
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_params(&mut self, params: PeriodicEventStreamParamsType) -> &mut Self {
        self.params = params;
        self
    }

    /// Gets the variable monitoring ID.
    ///
    /// # Returns
    ///
    /// The ID of monitor used to report this event
    pub fn variable_monitoring_id(&self) -> i32 {
        self.variable_monitoring_id
    }

    /// Sets the variable monitoring ID.
    ///
    /// # Arguments
    ///
    /// * `variable_monitoring_id` - Id of monitor used to report this event
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_monitoring_id(&mut self, variable_monitoring_id: i32) -> &mut Self {
        self.variable_monitoring_id = variable_monitoring_id;
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
    /// * `custom_data` - Custom data for this constant stream data, or None to clear
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
    fn test_new_constant_stream_data() {
        let params = PeriodicEventStreamParamsType::new(60);

        let stream_data = ConstantStreamDataType::new(1, params.clone(), 2);

        assert_eq!(stream_data.id(), 1);
        assert_eq!(stream_data.params().reporting_interval, 60);
        assert_eq!(stream_data.variable_monitoring_id(), 2);
        assert_eq!(stream_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let params = PeriodicEventStreamParamsType::new(60);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let stream_data =
            ConstantStreamDataType::new(1, params.clone(), 2).with_custom_data(custom_data.clone());

        assert_eq!(stream_data.id(), 1);
        assert_eq!(stream_data.params().reporting_interval, 60);
        assert_eq!(stream_data.variable_monitoring_id(), 2);
        assert_eq!(stream_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let params1 = PeriodicEventStreamParamsType::new(60);
        let params2 = PeriodicEventStreamParamsType::new(120);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut stream_data = ConstantStreamDataType::new(1, params1.clone(), 2);

        stream_data
            .set_id(3)
            .set_params(params2.clone())
            .set_variable_monitoring_id(4)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(stream_data.id(), 3);
        assert_eq!(stream_data.params().reporting_interval, 120);
        assert_eq!(stream_data.variable_monitoring_id(), 4);
        assert_eq!(stream_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        stream_data.set_custom_data(None);
        assert_eq!(stream_data.custom_data(), None);
    }
}
