use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType};
use crate::v2_1::enumerations::{
    DisplayMessageStatusEnumType, MessagePriorityEnumType, MessageStateEnumType,
};

/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier that identifies this message.
    #[validate(length(max = 50))]
    pub id: String,

    /// Required. Priority with which this message should be shown.
    pub priority: MessagePriorityEnumType,

    /// Required. Current state of this message.
    pub state: MessageStateEnumType,

    /// Required. Date and time at which this message was received.
    pub start_timestamp: DateTime<Utc>,

    /// Optional. Date and time at which this message should be removed from the display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<DateTime<Utc>>,

    /// Optional. Transaction Id for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Optional. Message details for a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional. Display style of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<DisplayMessageStatusEnumType>,

    /// Optional. Identification of the token for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
}

impl MessageInfoType {
    /// Creates a new `MessageInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier that identifies this message
    /// * `priority` - Priority with which this message should be shown
    /// * `state` - Current state of this message
    /// * `start_timestamp` - Date and time at which this message was received
    ///
    /// # Returns
    ///
    /// A new instance of `MessageInfoType` with optional fields set to `None`
    pub fn new(
        id: String,
        priority: MessagePriorityEnumType,
        state: MessageStateEnumType,
        start_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            priority,
            state,
            start_timestamp,
            end_timestamp: None,
            transaction_id: None,
            message: None,
            display: None,
            id_token: None,
            custom_data: None,
        }
    }

    /// Sets the end timestamp.
    ///
    /// # Arguments
    ///
    /// * `end_timestamp` - Date and time at which this message should be removed from the display
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_end_timestamp(mut self, end_timestamp: DateTime<Utc>) -> Self {
        self.end_timestamp = Some(end_timestamp);
        self
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - Transaction Id for which this message is intended
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_transaction_id(mut self, transaction_id: String) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Sets the message.
    ///
    /// # Arguments
    ///
    /// * `message` - Message details for a specific user
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets the display style.
    ///
    /// # Arguments
    ///
    /// * `display` - Display style of the message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_display(mut self, display: DisplayMessageStatusEnumType) -> Self {
        self.display = Some(display);
        self
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - Identification of the token for which this message is intended
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this message info
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the message identifier.
    ///
    /// # Returns
    ///
    /// The identifier that identifies this message
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the message identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier that identifies this message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the message priority.
    ///
    /// # Returns
    ///
    /// The priority with which this message should be shown
    pub fn priority(&self) -> &MessagePriorityEnumType {
        &self.priority
    }

    /// Sets the message priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority with which this message should be shown
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: MessagePriorityEnumType) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the message state.
    ///
    /// # Returns
    ///
    /// The current state of this message
    pub fn state(&self) -> &MessageStateEnumType {
        &self.state
    }

    /// Sets the message state.
    ///
    /// # Arguments
    ///
    /// * `state` - Current state of this message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_state(&mut self, state: MessageStateEnumType) -> &mut Self {
        self.state = state;
        self
    }

    /// Gets the start timestamp.
    ///
    /// # Returns
    ///
    /// The date and time at which this message was received
    pub fn start_timestamp(&self) -> &DateTime<Utc> {
        &self.start_timestamp
    }

    /// Sets the start timestamp.
    ///
    /// # Arguments
    ///
    /// * `start_timestamp` - Date and time at which this message was received
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_timestamp(&mut self, start_timestamp: DateTime<Utc>) -> &mut Self {
        self.start_timestamp = start_timestamp;
        self
    }

    /// Gets the end timestamp.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which this message should be removed
    pub fn end_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.end_timestamp.as_ref()
    }

    /// Sets the end timestamp.
    ///
    /// # Arguments
    ///
    /// * `end_timestamp` - Date and time at which this message should be removed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_end_timestamp(&mut self, end_timestamp: Option<DateTime<Utc>>) -> &mut Self {
        self.end_timestamp = end_timestamp;
        self
    }

    /// Gets the transaction ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the transaction Id for which this message is intended
    pub fn transaction_id(&self) -> Option<&str> {
        self.transaction_id.as_deref()
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - Transaction Id for which this message is intended, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction_id(&mut self, transaction_id: Option<String>) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Gets the message content.
    ///
    /// # Returns
    ///
    /// An optional reference to the message details for a specific user
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Sets the message content.
    ///
    /// # Arguments
    ///
    /// * `message` - Message details for a specific user, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_message(&mut self, message: Option<String>) -> &mut Self {
        self.message = message;
        self
    }

    /// Gets the display style.
    ///
    /// # Returns
    ///
    /// An optional reference to the display style of the message
    pub fn display(&self) -> Option<&DisplayMessageStatusEnumType> {
        self.display.as_ref()
    }

    /// Sets the display style.
    ///
    /// # Arguments
    ///
    /// * `display` - Display style of the message, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_display(&mut self, display: Option<DisplayMessageStatusEnumType>) -> &mut Self {
        self.display = display;
        self
    }

    /// Gets the ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identification of the token for which this message is intended
    pub fn id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - Identification of the token for which this message is intended, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
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
    /// * `custom_data` - Custom data for this message info, or None to clear
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
    fn test_new_message_info() {
        let id = "MSG001".to_string();
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();

        let message_info =
            MessageInfoType::new(id.clone(), priority.clone(), state.clone(), start_timestamp);

        assert_eq!(message_info.id(), id);
        assert_eq!(message_info.priority(), &priority);
        assert_eq!(message_info.state(), &state);
        assert_eq!(message_info.start_timestamp(), &start_timestamp);
        assert_eq!(message_info.end_timestamp(), None);
        assert_eq!(message_info.transaction_id(), None);
        assert_eq!(message_info.message(), None);
        assert_eq!(message_info.display(), None);
        assert_eq!(message_info.id_token(), None);
        assert_eq!(message_info.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "MSG001".to_string();
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();
        let end_timestamp = start_timestamp + chrono::Duration::hours(1);
        let transaction_id = "TX001".to_string();
        let message = "Please plug in your vehicle.".to_string();
        let display = DisplayMessageStatusEnumType::Accepted;
        let id_token = IdTokenType::new("TAG123".to_string(), "RFID".to_string());
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let message_info =
            MessageInfoType::new(id.clone(), priority.clone(), state.clone(), start_timestamp)
                .with_end_timestamp(end_timestamp)
                .with_transaction_id(transaction_id.clone())
                .with_message(message.clone())
                .with_display(display.clone())
                .with_id_token(id_token.clone())
                .with_custom_data(custom_data.clone());

        assert_eq!(message_info.id(), id);
        assert_eq!(message_info.priority(), &priority);
        assert_eq!(message_info.state(), &state);
        assert_eq!(message_info.start_timestamp(), &start_timestamp);
        assert_eq!(message_info.end_timestamp(), Some(&end_timestamp));
        assert_eq!(message_info.transaction_id(), Some(transaction_id.as_str()));
        assert_eq!(message_info.message(), Some(message.as_str()));
        assert_eq!(message_info.display(), Some(&display));
        assert_eq!(message_info.id_token(), Some(&id_token));
        assert_eq!(message_info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "MSG001".to_string();
        let id2 = "MSG002".to_string();
        let priority1 = MessagePriorityEnumType::AlwaysFront;
        let priority2 = MessagePriorityEnumType::InFront;
        let state1 = MessageStateEnumType::Charging;
        let state2 = MessageStateEnumType::Faulted;
        let start_timestamp1 = Utc::now();
        let start_timestamp2 = start_timestamp1 + chrono::Duration::hours(2);
        let end_timestamp = start_timestamp1 + chrono::Duration::hours(1);
        let transaction_id = "TX001".to_string();
        let message = "Please plug in your vehicle.".to_string();
        let display = DisplayMessageStatusEnumType::Accepted;
        let id_token = IdTokenType::new("TAG123".to_string(), "RFID".to_string());
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut message_info = MessageInfoType::new(
            id1.clone(),
            priority1.clone(),
            state1.clone(),
            start_timestamp1,
        );

        message_info
            .set_id(id2.clone())
            .set_priority(priority2.clone())
            .set_state(state2.clone())
            .set_start_timestamp(start_timestamp2)
            .set_end_timestamp(Some(end_timestamp))
            .set_transaction_id(Some(transaction_id.clone()))
            .set_message(Some(message.clone()))
            .set_display(Some(display.clone()))
            .set_id_token(Some(id_token.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(message_info.id(), id2);
        assert_eq!(message_info.priority(), &priority2);
        assert_eq!(message_info.state(), &state2);
        assert_eq!(message_info.start_timestamp(), &start_timestamp2);
        assert_eq!(message_info.end_timestamp(), Some(&end_timestamp));
        assert_eq!(message_info.transaction_id(), Some(transaction_id.as_str()));
        assert_eq!(message_info.message(), Some(message.as_str()));
        assert_eq!(message_info.display(), Some(&display));
        assert_eq!(message_info.id_token(), Some(&id_token));
        assert_eq!(message_info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        message_info
            .set_end_timestamp(None)
            .set_transaction_id(None)
            .set_message(None)
            .set_display(None)
            .set_id_token(None)
            .set_custom_data(None);

        assert_eq!(message_info.end_timestamp(), None);
        assert_eq!(message_info.transaction_id(), None);
        assert_eq!(message_info.message(), None);
        assert_eq!(message_info.display(), None);
        assert_eq!(message_info.id_token(), None);
        assert_eq!(message_info.custom_data(), None);
    }
}
