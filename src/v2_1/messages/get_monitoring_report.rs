use crate::v2_1::datatypes::{ComponentVariableType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{GenericDeviceModelStatusEnumType, MonitoringCriterionEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetMonitoringReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub component_variable: Option<Vec<ComponentVariableType>>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// This field contains criteria for components for which a monitoring report is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 3))]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetMonitoringReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            component_variable: None,
            request_id,
            monitoring_criteria: None,
            custom_data: None,
        }
    }

    /// Sets the component_variable field.
    ///
    /// * `component_variable` - The component_variable field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_component_variable(&mut self, component_variable: Option<Vec<ComponentVariableType>>) -> &mut Self {
        self.component_variable = component_variable;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the monitoring_criteria field.
    ///
    /// * `monitoring_criteria` - This field contains criteria for components for which a monitoring report is requested
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_monitoring_criteria(&mut self, monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>) -> &mut Self {
        self.monitoring_criteria = monitoring_criteria;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the component_variable field.
    ///
    /// # Returns
    ///
    /// The component_variable field
    pub fn get_component_variable(&self) -> Option<&Vec<ComponentVariableType>> {
        self.component_variable.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the monitoring_criteria field.
    ///
    /// # Returns
    ///
    /// This field contains criteria for components for which a monitoring report is requested
    pub fn get_monitoring_criteria(&self) -> Option<&Vec<MonitoringCriterionEnumType>> {
        self.monitoring_criteria.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the component_variable field and returns self for builder pattern.
    ///
    /// * `component_variable` - The component_variable field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_component_variable(mut self, component_variable: Vec<ComponentVariableType>) -> Self {
        self.component_variable = Some(component_variable);
        self
    }

    /// Sets the monitoring_criteria field and returns self for builder pattern.
    ///
    /// * `monitoring_criteria` - This field contains criteria for components for which a monitoring report is requested
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_monitoring_criteria(mut self, monitoring_criteria: Vec<MonitoringCriterionEnumType>) -> Self {
        self.monitoring_criteria = Some(monitoring_criteria);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the GetMonitoringReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetMonitoringReportResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericDeviceModelStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: GenericDeviceModelStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &GenericDeviceModelStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
