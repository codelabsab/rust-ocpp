use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::status_info::StatusInfoType;
use crate::v2_1::enumerations::ClearMonitoringStatusEnumType;

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Result of the clear request for this monitor, identified by its Id.
    pub status: ClearMonitoringStatusEnumType,

    /// Required. Id of the monitor of which a clear was requested.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl ClearMonitoringResultType {
    /// Creates a new `ClearMonitoringResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Result of the clear request for this monitor
    /// * `id` - Id of the monitor of which a clear was requested
    ///
    /// # Returns
    ///
    /// A new instance of `ClearMonitoringResultType` with optional fields set to `None`
    pub fn new(status: ClearMonitoringStatusEnumType, id: i32) -> Self {
        Self {
            status,
            id,
            custom_data: None,
            status_info: None,
        }
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear monitoring result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// The result of the clear request for this monitor
    pub fn status(&self) -> &ClearMonitoringStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Result of the clear request for this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: ClearMonitoringStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the monitor ID.
    ///
    /// # Returns
    ///
    /// The ID of the monitor of which a clear was requested
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the monitor ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the monitor of which a clear was requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the element providing more information about the status
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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
    /// * `custom_data` - Custom data for this clear monitoring result, or None to clear
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
    fn test_new_clear_monitoring_result() {
        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42);

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::Accepted);
        assert_eq!(result.id(), 42);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Rejected, 42)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::Rejected);
        assert_eq!(result.id(), 42);
        assert_eq!(result.status_info().unwrap().reason_code, "SomeReason");
        assert_eq!(result.status_info().unwrap().additional_info, Some("Additional details".to_string()));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42);

        result
            .set_status(ClearMonitoringStatusEnumType::NotFound)
            .set_id(43)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::NotFound);
        assert_eq!(result.id(), 43);
        assert_eq!(result.status_info().unwrap().reason_code, "SomeReason");
        assert_eq!(result.status_info().unwrap().additional_info, Some("Additional details".to_string()));
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }
}
