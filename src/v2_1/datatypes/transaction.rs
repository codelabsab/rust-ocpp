use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType};
use crate::v2_1::enumerations::ChargingStateEnumType;

/// A transaction for charging an EV.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the identifier by which the Charging Station and the CSMS identify the transaction.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Optional. The identifier that identifies the current charging state of the charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,

    /// Optional. The time when the transaction was started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_start: Option<DateTime<Utc>>,

    /// Optional. The time when the transaction was stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_end: Option<DateTime<Utc>>,

    /// Optional. This contains the identifier by which the user that started the transaction can be identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    /// Optional. This contains the identifier by which the user that stopped the transaction can be identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_id_token: Option<IdTokenType>,
}

impl TransactionType {
    /// Creates a new `TransactionType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier by which the Charging Station and the CSMS identify the transaction
    ///
    /// # Returns
    ///
    /// A new instance of `TransactionType` with optional fields set to `None`
    pub fn new(transaction_id: String) -> Self {
        Self {
            transaction_id,
            custom_data: None,
            charging_state: None,
            time_start: None,
            time_end: None,
            id_token: None,
            stop_id_token: None,
        }
    }

    /// Gets the transaction ID.
    ///
    /// # Returns
    ///
    /// The identifier by which the Charging Station and the CSMS identify the transaction
    pub fn transaction_id(&self) -> &str {
        &self.transaction_id
    }

    /// Gets the charging state.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier that identifies the current charging state of the charging session
    pub fn charging_state(&self) -> Option<&ChargingStateEnumType> {
        self.charging_state.as_ref()
    }

    /// Gets the start time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time when the transaction was started
    pub fn time_start(&self) -> Option<&DateTime<Utc>> {
        self.time_start.as_ref()
    }

    /// Gets the end time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time when the transaction was stopped
    pub fn time_end(&self) -> Option<&DateTime<Utc>> {
        self.time_end.as_ref()
    }

    /// Gets the ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier by which the user that started the transaction can be identified
    pub fn id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Gets the stop ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier by which the user that stopped the transaction can be identified
    pub fn stop_id_token(&self) -> Option<&IdTokenType> {
        self.stop_id_token.as_ref()
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier by which the Charging Station and the CSMS identify the transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Sets the charging state.
    ///
    /// # Arguments
    ///
    /// * `charging_state` - The identifier that identifies the current charging state of the charging session, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_state(
        &mut self,
        charging_state: Option<ChargingStateEnumType>,
    ) -> &mut Self {
        self.charging_state = charging_state;
        self
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `time_start` - The time when the transaction was started, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_start(&mut self, time_start: Option<DateTime<Utc>>) -> &mut Self {
        self.time_start = time_start;
        self
    }

    /// Sets the end time.
    ///
    /// # Arguments
    ///
    /// * `time_end` - The time when the transaction was stopped, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_end(&mut self, time_end: Option<DateTime<Utc>>) -> &mut Self {
        self.time_end = time_end;
        self
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - The identifier by which the user that started the transaction can be identified, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the stop ID token.
    ///
    /// # Arguments
    ///
    /// * `stop_id_token` - The identifier by which the user that stopped the transaction can be identified, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stop_id_token(&mut self, stop_id_token: Option<IdTokenType>) -> &mut Self {
        self.stop_id_token = stop_id_token;
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the charging state.
    ///
    /// # Arguments
    ///
    /// * `charging_state` - The identifier that identifies the current charging state of the charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_state(mut self, charging_state: ChargingStateEnumType) -> Self {
        self.charging_state = Some(charging_state);
        self
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `time_start` - The time when the transaction was started
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_time_start(mut self, time_start: DateTime<Utc>) -> Self {
        self.time_start = Some(time_start);
        self
    }

    /// Sets the end time.
    ///
    /// # Arguments
    ///
    /// * `time_end` - The time when the transaction was stopped
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_time_end(mut self, time_end: DateTime<Utc>) -> Self {
        self.time_end = Some(time_end);
        self
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - The identifier by which the user that started the transaction can be identified
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Sets the stop ID token.
    ///
    /// # Arguments
    ///
    /// * `stop_id_token` - The identifier by which the user that stopped the transaction can be identified
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stop_id_token(mut self, stop_id_token: IdTokenType) -> Self {
        self.stop_id_token = Some(stop_id_token);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let transaction_id = "TX12345".to_string();
        let transaction = TransactionType::new(transaction_id.clone());

        assert_eq!(transaction.transaction_id(), transaction_id);
        assert_eq!(transaction.charging_state(), None);
        assert_eq!(transaction.time_start(), None);
        assert_eq!(transaction.time_end(), None);
        assert_eq!(transaction.id_token(), None);
        assert_eq!(transaction.stop_id_token(), None);
        assert_eq!(transaction.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let transaction_id = "TX12345".to_string();
        let charging_state = ChargingStateEnumType::Charging;
        let time_start = Utc::now();
        let time_end = Utc::now();
        let id_token = IdTokenType::new("USER123".to_string(), "Local".to_string());
        let stop_id_token = IdTokenType::new("USER456".to_string(), "Local".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let transaction = TransactionType::new(transaction_id.clone())
            .with_charging_state(charging_state.clone())
            .with_time_start(time_start.clone())
            .with_time_end(time_end.clone())
            .with_id_token(id_token.clone())
            .with_stop_id_token(stop_id_token.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(transaction.transaction_id(), transaction_id);
        assert_eq!(transaction.charging_state(), Some(&charging_state));
        assert_eq!(transaction.time_start(), Some(&time_start));
        assert_eq!(transaction.time_end(), Some(&time_end));
        assert_eq!(transaction.id_token(), Some(&id_token));
        assert_eq!(transaction.stop_id_token(), Some(&stop_id_token));
        assert_eq!(transaction.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let transaction_id1 = "TX12345".to_string();
        let transaction_id2 = "TX67890".to_string();
        let charging_state = ChargingStateEnumType::Charging;
        let time_start = Utc::now();
        let time_end = Utc::now();
        let id_token = IdTokenType::new("USER123".to_string(), "Local".to_string());
        let stop_id_token = IdTokenType::new("USER456".to_string(), "Local".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut transaction = TransactionType::new(transaction_id1);

        transaction
            .set_transaction_id(transaction_id2.clone())
            .set_charging_state(Some(charging_state.clone()))
            .set_time_start(Some(time_start.clone()))
            .set_time_end(Some(time_end.clone()))
            .set_id_token(Some(id_token.clone()))
            .set_stop_id_token(Some(stop_id_token.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(transaction.transaction_id(), transaction_id2);
        assert_eq!(transaction.charging_state(), Some(&charging_state));
        assert_eq!(transaction.time_start(), Some(&time_start));
        assert_eq!(transaction.time_end(), Some(&time_end));
        assert_eq!(transaction.id_token(), Some(&id_token));
        assert_eq!(transaction.stop_id_token(), Some(&stop_id_token));
        assert_eq!(transaction.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        transaction
            .set_charging_state(None)
            .set_time_start(None)
            .set_time_end(None)
            .set_id_token(None)
            .set_stop_id_token(None)
            .set_custom_data(None);

        assert_eq!(transaction.transaction_id(), transaction_id2);
        assert_eq!(transaction.charging_state(), None);
        assert_eq!(transaction.time_start(), None);
        assert_eq!(transaction.time_end(), None);
        assert_eq!(transaction.id_token(), None);
        assert_eq!(transaction.stop_id_token(), None);
        assert_eq!(transaction.custom_data(), None);
    }
}
