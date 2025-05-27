use crate::v2_1::datatypes::{CustomDataType, IdTokenType, StatusInfoType};
use crate::v2_1::enumerations::ReserveNowStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ReserveNow request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    /// Id of reservation.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Date and time at which the reservation expires.
    pub expiry_date_time: DateTime<Utc>,

    /// This field specifies the connector type. Values defined in Appendix as ConnectorEnumStringType.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub connector_type: Option<String>,

    #[validate(nested)]
    pub id_token: IdTokenType,

    /// This contains ID of the evse to be reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub group_id_token: Option<IdTokenType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReserveNowRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - Id of reservation.
    /// * `expiry_date_time` - Date and time at which the reservation expires.
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id: i32, expiry_date_time: DateTime<Utc>, id_token: IdTokenType) -> Self {
        Self {
            id,
            expiry_date_time,
            connector_type: None,
            id_token,
            evse_id: None,
            group_id_token: None,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - Id of reservation.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets the expiry_date_time field.
    ///
    /// * `expiry_date_time` - Date and time at which the reservation expires.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_expiry_date_time(&mut self, expiry_date_time: DateTime<Utc>) -> &mut Self {
        self.expiry_date_time = expiry_date_time;
        self
    }

    /// Sets the connector_type field.
    ///
    /// * `connector_type` - This field specifies the connector type. Values defined in Appendix as ConnectorEnumStringType.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_type(&mut self, connector_type: Option<String>) -> &mut Self {
        self.connector_type = connector_type;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - This contains ID of the evse to be reserved.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the group_id_token field.
    ///
    /// * `group_id_token` - The group_id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_group_id_token(&mut self, group_id_token: Option<IdTokenType>) -> &mut Self {
        self.group_id_token = group_id_token;
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

    /// Gets a reference to the id field.
    ///
    /// # Returns
    ///
    /// Id of reservation.
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    /// Gets a reference to the expiry_date_time field.
    ///
    /// # Returns
    ///
    /// Date and time at which the reservation expires.
    pub fn get_expiry_date_time(&self) -> &DateTime<Utc> {
        &self.expiry_date_time
    }

    /// Gets a reference to the connector_type field.
    ///
    /// # Returns
    ///
    /// This field specifies the connector type. Values defined in Appendix as ConnectorEnumStringType.
    pub fn get_connector_type(&self) -> Option<&String> {
        self.connector_type.as_ref()
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// This contains ID of the evse to be reserved.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the group_id_token field.
    ///
    /// # Returns
    ///
    /// The group_id_token field
    pub fn get_group_id_token(&self) -> Option<&IdTokenType> {
        self.group_id_token.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the connector_type field and returns self for builder pattern.
    ///
    /// * `connector_type` - This field specifies the connector type. Values defined in Appendix as ConnectorEnumStringType.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_connector_type(mut self, connector_type: String) -> Self {
        self.connector_type = Some(connector_type);
        self
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - This contains ID of the evse to be reserved.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }

    /// Sets the group_id_token field and returns self for builder pattern.
    ///
    /// * `group_id_token` - The group_id_token field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_group_id_token(mut self, group_id_token: IdTokenType) -> Self {
        self.group_id_token = Some(group_id_token);
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

/// Response body for the ReserveNow response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowResponse {
    pub status: ReserveNowStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReserveNowResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ReserveNowStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ReserveNowStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ReserveNowStatusEnumType {
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
