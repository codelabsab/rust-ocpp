use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetTransactionStatus request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    /// The Id of the transaction for which the status is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTransactionStatusRequest {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            transaction_id: None,
            custom_data: None,
        }
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - The Id of the transaction for which the status is requested.
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

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// The Id of the transaction for which the status is requested.
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

    /// Sets the transaction_id field and returns self for builder pattern.
    ///
    /// * `transaction_id` - The Id of the transaction for which the status is requested.
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

/// Response body for the GetTransactionStatus response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusResponse {
    /// Whether the transaction is still ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,

    /// Whether there are still message to be delivered.
    pub messages_in_queue: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTransactionStatusResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `messages_in_queue` - Whether there are still message to be delivered.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(messages_in_queue: bool) -> Self {
        Self {
            ongoing_indicator: None,
            messages_in_queue,
            custom_data: None,
        }
    }

    /// Sets the ongoing_indicator field.
    ///
    /// * `ongoing_indicator` - Whether the transaction is still ongoing.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_ongoing_indicator(&mut self, ongoing_indicator: Option<bool>) -> &mut Self {
        self.ongoing_indicator = ongoing_indicator;
        self
    }

    /// Sets the messages_in_queue field.
    ///
    /// * `messages_in_queue` - Whether there are still message to be delivered.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_messages_in_queue(&mut self, messages_in_queue: bool) -> &mut Self {
        self.messages_in_queue = messages_in_queue;
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

    /// Gets a reference to the ongoing_indicator field.
    ///
    /// # Returns
    ///
    /// Whether the transaction is still ongoing.
    pub fn get_ongoing_indicator(&self) -> Option<&bool> {
        self.ongoing_indicator.as_ref()
    }

    /// Gets a reference to the messages_in_queue field.
    ///
    /// # Returns
    ///
    /// Whether there are still message to be delivered.
    pub fn get_messages_in_queue(&self) -> &bool {
        &self.messages_in_queue
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the ongoing_indicator field and returns self for builder pattern.
    ///
    /// * `ongoing_indicator` - Whether the transaction is still ongoing.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_ongoing_indicator(mut self, ongoing_indicator: bool) -> Self {
        self.ongoing_indicator = Some(ongoing_indicator);
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
