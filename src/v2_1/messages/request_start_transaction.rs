use crate::v2_1::datatypes::{
    ChargingProfileType, 
    CustomDataType, 
    IdTokenType, 
    StatusInfoType,
};
use crate::v2_1::enumerations::RequestStartStopStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the StartTransaction request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    /// Number of the EVSE on which to start the transaction. EvseId SHALL be &gt; 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub group_id_token: Option<IdTokenType>,

    #[validate(nested)]
    pub id_token: IdTokenType,

    /// Id given by the server to this start request. The Charging Station will return this in the &lt;&lt;transactioneventrequest, TransactionEventRequest&gt;&gt;, letting the server know which transaction was started for this request. Use to start a transaction.
    #[validate(range(min = 0))]
    pub remote_start_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub charging_profile: Option<ChargingProfileType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestStartTransactionRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id_token` - The id_token field
    /// * `remote_start_id` - Id given by the server to this start request. The Charging Station will return this in the &lt;&lt;transactioneventrequest, TransactionEventRequest&gt;&gt;, letting the server know which transaction was started for this request. Use to start a transaction.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id_token: IdTokenType, remote_start_id: i32) -> Self {
        Self {
            evse_id: None,
            group_id_token: None,
            id_token,
            remote_start_id,
            charging_profile: None,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - Number of the EVSE on which to start the transaction. EvseId SHALL be &gt; 0
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

    /// Sets the remote_start_id field.
    ///
    /// * `remote_start_id` - Id given by the server to this start request. The Charging Station will return this in the &lt;&lt;transactioneventrequest, TransactionEventRequest&gt;&gt;, letting the server know which transaction was started for this request. Use to start a transaction.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_remote_start_id(&mut self, remote_start_id: i32) -> &mut Self {
        self.remote_start_id = remote_start_id;
        self
    }

    /// Sets the charging_profile field.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile(&mut self, charging_profile: Option<ChargingProfileType>) -> &mut Self {
        self.charging_profile = charging_profile;
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// Number of the EVSE on which to start the transaction. EvseId SHALL be &gt; 0
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

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the remote_start_id field.
    ///
    /// # Returns
    ///
    /// Id given by the server to this start request. The Charging Station will return this in the &lt;&lt;transactioneventrequest, TransactionEventRequest&gt;&gt;, letting the server know which transaction was started for this request. Use to start a transaction.
    pub fn get_remote_start_id(&self) -> &i32 {
        &self.remote_start_id
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> Option<&ChargingProfileType> {
        self.charging_profile.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - Number of the EVSE on which to start the transaction. EvseId SHALL be &gt; 0
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

    /// Sets the charging_profile field and returns self for builder pattern.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_profile(mut self, charging_profile: ChargingProfileType) -> Self {
        self.charging_profile = Some(charging_profile);
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

/// Response body for the StartTransaction response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionResponse {
    pub status: RequestStartStopStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// When the transaction was already started by the Charging Station before the RequestStartTransactionRequest was received, for example: cable plugged in first. This contains the transactionId of the already started transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestStartTransactionResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: RequestStartStopStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            transaction_id: None,
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
    pub fn set_status(&mut self, status: RequestStartStopStatusEnumType) -> &mut Self {
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

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - When the transaction was already started by the Charging Station before the RequestStartTransactionRequest was received, for example: cable plugged in first. This contains the transactionId of the already started transaction.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: Option<String>) -> &mut Self {
        self.transaction_id = transaction_id;
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
    pub fn get_status(&self) -> &RequestStartStopStatusEnumType {
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

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// When the transaction was already started by the Charging Station before the RequestStartTransactionRequest was received, for example: cable plugged in first. This contains the transactionId of the already started transaction.
    pub fn get_transaction_id(&self) -> Option<&String> {
        self.transaction_id.as_ref()
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

    /// Sets the transaction_id field and returns self for builder pattern.
    ///
    /// * `transaction_id` - When the transaction was already started by the Charging Station before the RequestStartTransactionRequest was received, for example: cable plugged in first. This contains the transactionId of the already started transaction.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_transaction_id(mut self, transaction_id: String) -> Self {
        self.transaction_id = Some(transaction_id);
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
