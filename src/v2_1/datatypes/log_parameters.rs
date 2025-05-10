use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Log parameters for GetLog request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Id of this request.
    #[validate(length(max = 36))]
    pub remote_location: String,

    /// Required. The oldest log entry date/time to include in the response.
    pub oldest_timestamp: String,

    /// Optional. The latest log entry date/time to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<String>,
}

impl LogParametersType {
    /// Creates a new `LogParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `remote_location` - The Id of this request
    /// * `oldest_timestamp` - The oldest log entry date/time to include in the response
    ///
    /// # Returns
    ///
    /// A new instance of `LogParametersType` with optional fields set to `None`
    pub fn new(remote_location: String, oldest_timestamp: String) -> Self {
        Self {
            remote_location,
            oldest_timestamp,
            latest_timestamp: None,
            custom_data: None,
        }
    }

    /// Sets the latest timestamp.
    ///
    /// # Arguments
    ///
    /// * `latest_timestamp` - The latest log entry date/time to include in the response
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_latest_timestamp(mut self, latest_timestamp: String) -> Self {
        self.latest_timestamp = Some(latest_timestamp);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this log parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the remote location.
    ///
    /// # Returns
    ///
    /// The Id of this request
    pub fn remote_location(&self) -> &str {
        &self.remote_location
    }

    /// Sets the remote location.
    ///
    /// # Arguments
    ///
    /// * `remote_location` - The Id of this request
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_remote_location(&mut self, remote_location: String) -> &mut Self {
        self.remote_location = remote_location;
        self
    }

    /// Gets the oldest timestamp.
    ///
    /// # Returns
    ///
    /// The oldest log entry date/time to include in the response
    pub fn oldest_timestamp(&self) -> &str {
        &self.oldest_timestamp
    }

    /// Sets the oldest timestamp.
    ///
    /// # Arguments
    ///
    /// * `oldest_timestamp` - The oldest log entry date/time to include in the response
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_oldest_timestamp(&mut self, oldest_timestamp: String) -> &mut Self {
        self.oldest_timestamp = oldest_timestamp;
        self
    }

    /// Gets the latest timestamp.
    ///
    /// # Returns
    ///
    /// An optional reference to the latest log entry date/time to include in the response
    pub fn latest_timestamp(&self) -> Option<&str> {
        self.latest_timestamp.as_deref()
    }

    /// Sets the latest timestamp.
    ///
    /// # Arguments
    ///
    /// * `latest_timestamp` - The latest log entry date/time to include in the response, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_latest_timestamp(&mut self, latest_timestamp: Option<String>) -> &mut Self {
        self.latest_timestamp = latest_timestamp;
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
    /// * `custom_data` - Custom data for this log parameters, or None to clear
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
    fn test_new_log_parameters() {
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = "2023-01-01T00:00:00Z".to_string();

        let log_params = LogParametersType::new(remote_location.clone(), oldest_timestamp.clone());

        assert_eq!(log_params.remote_location(), remote_location);
        assert_eq!(log_params.oldest_timestamp(), oldest_timestamp);
        assert_eq!(log_params.latest_timestamp(), None);
        assert_eq!(log_params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = "2023-01-01T00:00:00Z".to_string();
        let latest_timestamp = "2023-01-31T23:59:59Z".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let log_params = LogParametersType::new(remote_location.clone(), oldest_timestamp.clone())
            .with_latest_timestamp(latest_timestamp.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(log_params.remote_location(), remote_location);
        assert_eq!(log_params.oldest_timestamp(), oldest_timestamp);
        assert_eq!(
            log_params.latest_timestamp(),
            Some(latest_timestamp.as_str())
        );
        assert_eq!(log_params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let remote_location1 = "https://example.com/logs".to_string();
        let remote_location2 = "https://logs.example.org".to_string();
        let oldest_timestamp1 = "2023-01-01T00:00:00Z".to_string();
        let oldest_timestamp2 = "2023-02-01T00:00:00Z".to_string();
        let latest_timestamp = "2023-01-31T23:59:59Z".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut log_params =
            LogParametersType::new(remote_location1.clone(), oldest_timestamp1.clone());

        log_params
            .set_remote_location(remote_location2.clone())
            .set_oldest_timestamp(oldest_timestamp2.clone())
            .set_latest_timestamp(Some(latest_timestamp.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(log_params.remote_location(), remote_location2);
        assert_eq!(log_params.oldest_timestamp(), oldest_timestamp2);
        assert_eq!(
            log_params.latest_timestamp(),
            Some(latest_timestamp.as_str())
        );
        assert_eq!(log_params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        log_params.set_latest_timestamp(None).set_custom_data(None);

        assert_eq!(log_params.latest_timestamp(), None);
        assert_eq!(log_params.custom_data(), None);
    }
}
