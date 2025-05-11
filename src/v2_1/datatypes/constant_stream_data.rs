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
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,

    /// Uniquely identifies the stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Parameters for the periodic event stream.
    #[validate(nested)]
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
    use serde_json::{json, Value};
    use validator::Validate;

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

    #[test]
    fn test_serialization() {
        let params = PeriodicEventStreamParamsType::new(60);
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let stream_data = ConstantStreamDataType::new(1, params, 2).with_custom_data(custom_data);

        let serialized = serde_json::to_string(&stream_data).unwrap();
        let deserialized: ConstantStreamDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(stream_data, deserialized);

        // Verify specific JSON structure
        let json_value: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["params"]["reportingInterval"], 60);
        assert_eq!(json_value["variableMonitoringId"], 2);
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization() {
        let json_str = r#"{
            "id": 5,
            "params": {
                "reportingInterval": 30
            },
            "variableMonitoringId": 10,
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        let stream_data: ConstantStreamDataType = serde_json::from_str(json_str).unwrap();

        assert_eq!(stream_data.id(), 5);
        assert_eq!(stream_data.params().reporting_interval(), 30);
        assert_eq!(stream_data.variable_monitoring_id(), 10);
        assert_eq!(stream_data.custom_data().unwrap().vendor_id(), "TestVendor");
        assert_eq!(
            stream_data.custom_data().unwrap().additional_properties()["extraInfo"],
            json!("Something")
        );
    }

    #[test]
    fn test_validation() {
        // Valid case
        let params = PeriodicEventStreamParamsType::new(60);
        let stream_data = ConstantStreamDataType::new(1, params.clone(), 2);
        assert!(stream_data.validate().is_ok());

        // Invalid id (negative)
        let invalid_id_data = ConstantStreamDataType::new(-1, params.clone(), 2);
        assert!(invalid_id_data.validate().is_err());

        // Invalid variable_monitoring_id (negative)
        let invalid_variable_id_data = ConstantStreamDataType::new(1, params.clone(), -5);
        assert!(invalid_variable_id_data.validate().is_err());

        // Invalid params (reporting_interval out of range)
        let invalid_params = PeriodicEventStreamParamsType::new(0); // Min is 1
        let invalid_params_data = ConstantStreamDataType::new(1, invalid_params, 2);
        assert!(invalid_params_data.validate().is_err());

        // Invalid params (reporting_interval too large)
        let too_large_params = PeriodicEventStreamParamsType::new(86401); // Max is 86400
        let too_large_params_data = ConstantStreamDataType::new(1, too_large_params, 2);
        assert!(too_large_params_data.validate().is_err());
    }

    #[test]
    fn test_complex_scenario() {
        // Create a complex scenario with custom data and parameter chains
        let vendor_custom_data = CustomDataType::new("VendorComplex".to_string())
            .with_property("version".to_string(), json!("2.5"))
            .with_property("features".to_string(), json!(["advanced", "premium"]))
            .with_property(
                "config".to_string(),
                json!({
                    "timeout": 120,
                    "retries": 3,
                    "enabled": true
                }),
            );

        let params_custom_data = CustomDataType::new("ParamsVendor".to_string())
            .with_property("mode".to_string(), json!("enhanced"));

        let params = PeriodicEventStreamParamsType::new(300).with_custom_data(params_custom_data);

        let stream_data =
            ConstantStreamDataType::new(42, params, 99).with_custom_data(vendor_custom_data);

        // Serialize and deserialize
        let serialized = serde_json::to_string(&stream_data).unwrap();
        let deserialized: ConstantStreamDataType = serde_json::from_str(&serialized).unwrap();

        // Verify the complex structure is preserved
        assert_eq!(deserialized.id(), 42);
        assert_eq!(deserialized.variable_monitoring_id(), 99);
        assert_eq!(deserialized.params().reporting_interval(), 300);

        let custom_data = deserialized.custom_data().unwrap();
        assert_eq!(custom_data.vendor_id(), "VendorComplex");
        assert_eq!(custom_data.additional_properties()["version"], json!("2.5"));

        let features = &custom_data.additional_properties()["features"];
        assert_eq!(features[0], "advanced");
        assert_eq!(features[1], "premium");

        let config = &custom_data.additional_properties()["config"];
        assert_eq!(config["timeout"], 120);
        assert_eq!(config["retries"], 3);
        assert_eq!(config["enabled"], true);

        let params_custom = deserialized.params().custom_data().unwrap();
        assert_eq!(params_custom.vendor_id(), "ParamsVendor");
        assert_eq!(params_custom.additional_properties()["mode"], "enhanced");
    }

    #[test]
    fn test_boundary_values() {
        // Test with minimum valid values
        let min_params = PeriodicEventStreamParamsType::new(1); // Minimum reporting interval
        let min_stream_data = ConstantStreamDataType::new(0, min_params, 0); // Minimum IDs

        assert_eq!(min_stream_data.id(), 0);
        assert_eq!(min_stream_data.params().reporting_interval(), 1);
        assert_eq!(min_stream_data.variable_monitoring_id(), 0);
        assert!(min_stream_data.validate().is_ok());

        // Test with maximum valid values for reporting interval
        let max_params = PeriodicEventStreamParamsType::new(86400); // Maximum reporting interval
        let max_stream_data = ConstantStreamDataType::new(i32::MAX, max_params, i32::MAX);

        assert_eq!(max_stream_data.id(), i32::MAX);
        assert_eq!(max_stream_data.params().reporting_interval(), 86400);
        assert_eq!(max_stream_data.variable_monitoring_id(), i32::MAX);
        assert!(max_stream_data.validate().is_ok());
    }
}
