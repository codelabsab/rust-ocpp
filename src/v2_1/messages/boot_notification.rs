use crate::v2_1::datatypes::{ChargingStationType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{BootReasonEnumType, RegistrationStatusEnumType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the BootNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    #[validate(nested)]
    pub charging_station: ChargingStationType,

    pub reason: BootReasonEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BootNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_station` - The charging_station field
    /// * `reason` - The reason field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_station: ChargingStationType, reason: BootReasonEnumType) -> Self {
        Self {
            charging_station,
            reason,
            custom_data: None,
        }
    }

    /// Sets the charging_station field.
    ///
    /// * `charging_station` - The charging_station field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_station(&mut self, charging_station: ChargingStationType) -> &mut Self {
        self.charging_station = charging_station;
        self
    }

    /// Sets the reason field.
    ///
    /// * `reason` - The reason field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reason(&mut self, reason: BootReasonEnumType) -> &mut Self {
        self.reason = reason;
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

    /// Gets a reference to the charging_station field.
    ///
    /// # Returns
    ///
    /// The charging_station field
    pub fn get_charging_station(&self) -> &ChargingStationType {
        &self.charging_station
    }

    /// Gets a reference to the reason field.
    ///
    /// # Returns
    ///
    /// The reason field
    pub fn get_reason(&self) -> &BootReasonEnumType {
        &self.reason
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

/// Response body for the BootNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// This contains the CSMS’s current time.
    pub current_time: DateTime<Utc>,

    /// When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    pub interval: i32,

    pub status: RegistrationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BootNotificationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `current_time` - This contains the CSMS’s current time.
    /// * `interval` - When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(current_time: DateTime<Utc>, interval: i32, status: RegistrationStatusEnumType) -> Self {
        Self {
            current_time,
            interval,
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the current_time field.
    ///
    /// * `current_time` - This contains the CSMS’s current time.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_current_time(&mut self, current_time: DateTime<Utc>) -> &mut Self {
        self.current_time = current_time;
        self
    }

    /// Sets the interval field.
    ///
    /// * `interval` - When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.interval = interval;
        self
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: RegistrationStatusEnumType) -> &mut Self {
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

    /// Gets a reference to the current_time field.
    ///
    /// # Returns
    ///
    /// This contains the CSMS’s current time.
    pub fn get_current_time(&self) -> &DateTime<Utc> {
        &self.current_time
    }

    /// Gets a reference to the interval field.
    ///
    /// # Returns
    ///
    /// When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    pub fn get_interval(&self) -> &i32 {
        &self.interval
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &RegistrationStatusEnumType {
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
