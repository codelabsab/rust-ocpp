use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::ConnectorStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the StatusNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    /// The time for which the status is reported.
    pub timestamp: DateTime<Utc>,

    pub connector_status: ConnectorStatusEnumType,

    /// The id of the EVSE to which the connector belongs for which the the status is reported.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// The id of the connector within the EVSE for which the status is reported.
    #[validate(range(min = 0))]
    pub connector_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StatusNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `timestamp` - The time for which the status is reported.
    /// * `connector_status` - The connector_status field
    /// * `evse_id` - The id of the EVSE to which the connector belongs for which the the status is reported.
    /// * `connector_id` - The id of the connector within the EVSE for which the status is reported.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(timestamp: DateTime<Utc>, connector_status: ConnectorStatusEnumType, evse_id: i32, connector_id: i32) -> Self {
        Self {
            timestamp,
            connector_status,
            evse_id,
            connector_id,
            custom_data: None,
        }
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - The time for which the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the connector_status field.
    ///
    /// * `connector_status` - The connector_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_status(&mut self, connector_status: ConnectorStatusEnumType) -> &mut Self {
        self.connector_status = connector_status;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The id of the EVSE to which the connector belongs for which the the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the connector_id field.
    ///
    /// * `connector_id` - The id of the connector within the EVSE for which the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_id(&mut self, connector_id: i32) -> &mut Self {
        self.connector_id = connector_id;
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

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// The time for which the status is reported.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the connector_status field.
    ///
    /// # Returns
    ///
    /// The connector_status field
    pub fn get_connector_status(&self) -> &ConnectorStatusEnumType {
        &self.connector_status
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The id of the EVSE to which the connector belongs for which the the status is reported.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the connector_id field.
    ///
    /// # Returns
    ///
    /// The id of the connector within the EVSE for which the status is reported.
    pub fn get_connector_id(&self) -> &i32 {
        &self.connector_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
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

/// Response body for the StatusNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StatusNotificationResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
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

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
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
