use crate::v2_1::datatypes::{
    CostDetailsType, 
    CustomDataType, 
    EVSEType, 
    IdTokenInfoType, 
    IdTokenType, 
    MessageContentType, 
    MeterValueType, 
    TransactionLimitType, 
    TransactionType,
};
use crate::v2_1::enumerations::{PreconditioningStatusEnumType, TransactionEventEnumType, TriggerReasonEnumType};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the TransactionEvent request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub cost_details: Option<CostDetailsType>,

    pub event_type: TransactionEventEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub meter_value: Option<Vec<MeterValueType>>,

    /// The date and time at which this transaction event occurred.
    pub timestamp: DateTime<Utc>,

    pub trigger_reason: TriggerReasonEnumType,

    /// Incremental sequence number, helps with determining if all messages of a transaction have been received.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Indication that this transaction event happened when the Charging Station was offline. Default = false, meaning: the event occurred when the Charging Station was online.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,

    /// If the Charging Station is able to report the number of phases used, then it SHALL provide it. When omitted the CSMS may be able to determine the number of phases used as follows: + 1: The numberPhases in the currently used ChargingSchedule. + 2: The number of phases provided via device management.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 3))]
    pub number_of_phases_used: Option<i32>,

    /// The maximum current of the connected cable in Ampere (A).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i32>,

    /// This contains the Id of the reservation that terminates as a result of this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub reservation_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditioning_status: Option<PreconditioningStatusEnumType>,

    /// *(2.1)* True when EVSE electronics are in sleep mode for this transaction. Default value (when absent) is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_sleep: Option<bool>,

    #[validate(nested)]
    pub transaction_info: TransactionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSEType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token: Option<IdTokenType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TransactionEventRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `event_type` - The event_type field
    /// * `timestamp` - The date and time at which this transaction event occurred.
    /// * `trigger_reason` - The trigger_reason field
    /// * `seq_no` - Incremental sequence number, helps with determining if all messages of a transaction have been received.
    /// * `transaction_info` - The transaction_info field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(event_type: TransactionEventEnumType, timestamp: DateTime<Utc>, trigger_reason: TriggerReasonEnumType, seq_no: i32, transaction_info: TransactionType) -> Self {
        Self {
            cost_details: None,
            event_type,
            meter_value: None,
            timestamp,
            trigger_reason,
            seq_no,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            preconditioning_status: None,
            evse_sleep: None,
            transaction_info,
            evse: None,
            id_token: None,
            custom_data: None,
        }
    }

    /// Sets the cost_details field.
    ///
    /// * `cost_details` - The cost_details field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_cost_details(&mut self, cost_details: Option<CostDetailsType>) -> &mut Self {
        self.cost_details = cost_details;
        self
    }

    /// Sets the event_type field.
    ///
    /// * `event_type` - The event_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_event_type(&mut self, event_type: TransactionEventEnumType) -> &mut Self {
        self.event_type = event_type;
        self
    }

    /// Sets the meter_value field.
    ///
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_meter_value(&mut self, meter_value: Option<Vec<MeterValueType>>) -> &mut Self {
        self.meter_value = meter_value;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - The date and time at which this transaction event occurred.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the trigger_reason field.
    ///
    /// * `trigger_reason` - The trigger_reason field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_trigger_reason(&mut self, trigger_reason: TriggerReasonEnumType) -> &mut Self {
        self.trigger_reason = trigger_reason;
        self
    }

    /// Sets the seq_no field.
    ///
    /// * `seq_no` - Incremental sequence number, helps with determining if all messages of a transaction have been received.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_seq_no(&mut self, seq_no: i32) -> &mut Self {
        self.seq_no = seq_no;
        self
    }

    /// Sets the offline field.
    ///
    /// * `offline` - Indication that this transaction event happened when the Charging Station was offline. Default = false, meaning: the event occurred when the Charging Station was online.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_offline(&mut self, offline: Option<bool>) -> &mut Self {
        self.offline = offline;
        self
    }

    /// Sets the number_of_phases_used field.
    ///
    /// * `number_of_phases_used` - If the Charging Station is able to report the number of phases used, then it SHALL provide it. When omitted the CSMS may be able to determine the number of phases used as follows: + 1: The numberPhases in the currently used ChargingSchedule. + 2: The number of phases provided via device management.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_number_of_phases_used(&mut self, number_of_phases_used: Option<i32>) -> &mut Self {
        self.number_of_phases_used = number_of_phases_used;
        self
    }

    /// Sets the cable_max_current field.
    ///
    /// * `cable_max_current` - The maximum current of the connected cable in Ampere (A).
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_cable_max_current(&mut self, cable_max_current: Option<i32>) -> &mut Self {
        self.cable_max_current = cable_max_current;
        self
    }

    /// Sets the reservation_id field.
    ///
    /// * `reservation_id` - This contains the Id of the reservation that terminates as a result of this transaction.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reservation_id(&mut self, reservation_id: Option<i32>) -> &mut Self {
        self.reservation_id = reservation_id;
        self
    }

    /// Sets the preconditioning_status field.
    ///
    /// * `preconditioning_status` - The preconditioning_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_preconditioning_status(&mut self, preconditioning_status: Option<PreconditioningStatusEnumType>) -> &mut Self {
        self.preconditioning_status = preconditioning_status;
        self
    }

    /// Sets the evse_sleep field.
    ///
    /// * `evse_sleep` - *(2.1)* True when EVSE electronics are in sleep mode for this transaction. Default value (when absent) is false.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_sleep(&mut self, evse_sleep: Option<bool>) -> &mut Self {
        self.evse_sleep = evse_sleep;
        self
    }

    /// Sets the transaction_info field.
    ///
    /// * `transaction_info` - The transaction_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_info(&mut self, transaction_info: TransactionType) -> &mut Self {
        self.transaction_info = transaction_info;
        self
    }

    /// Sets the evse field.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse(&mut self, evse: Option<EVSEType>) -> &mut Self {
        self.evse = evse;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
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

    /// Gets a reference to the cost_details field.
    ///
    /// # Returns
    ///
    /// The cost_details field
    pub fn get_cost_details(&self) -> Option<&CostDetailsType> {
        self.cost_details.as_ref()
    }

    /// Gets a reference to the event_type field.
    ///
    /// # Returns
    ///
    /// The event_type field
    pub fn get_event_type(&self) -> &TransactionEventEnumType {
        &self.event_type
    }

    /// Gets a reference to the meter_value field.
    ///
    /// # Returns
    ///
    /// The meter_value field
    pub fn get_meter_value(&self) -> Option<&Vec<MeterValueType>> {
        self.meter_value.as_ref()
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// The date and time at which this transaction event occurred.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the trigger_reason field.
    ///
    /// # Returns
    ///
    /// The trigger_reason field
    pub fn get_trigger_reason(&self) -> &TriggerReasonEnumType {
        &self.trigger_reason
    }

    /// Gets a reference to the seq_no field.
    ///
    /// # Returns
    ///
    /// Incremental sequence number, helps with determining if all messages of a transaction have been received.
    pub fn get_seq_no(&self) -> &i32 {
        &self.seq_no
    }

    /// Gets a reference to the offline field.
    ///
    /// # Returns
    ///
    /// Indication that this transaction event happened when the Charging Station was offline. Default = false, meaning: the event occurred when the Charging Station was online.
    pub fn get_offline(&self) -> Option<&bool> {
        self.offline.as_ref()
    }

    /// Gets a reference to the number_of_phases_used field.
    ///
    /// # Returns
    ///
    /// If the Charging Station is able to report the number of phases used, then it SHALL provide it. When omitted the CSMS may be able to determine the number of phases used as follows: + 1: The numberPhases in the currently used ChargingSchedule. + 2: The number of phases provided via device management.
    pub fn get_number_of_phases_used(&self) -> Option<&i32> {
        self.number_of_phases_used.as_ref()
    }

    /// Gets a reference to the cable_max_current field.
    ///
    /// # Returns
    ///
    /// The maximum current of the connected cable in Ampere (A).
    pub fn get_cable_max_current(&self) -> Option<&i32> {
        self.cable_max_current.as_ref()
    }

    /// Gets a reference to the reservation_id field.
    ///
    /// # Returns
    ///
    /// This contains the Id of the reservation that terminates as a result of this transaction.
    pub fn get_reservation_id(&self) -> Option<&i32> {
        self.reservation_id.as_ref()
    }

    /// Gets a reference to the preconditioning_status field.
    ///
    /// # Returns
    ///
    /// The preconditioning_status field
    pub fn get_preconditioning_status(&self) -> Option<&PreconditioningStatusEnumType> {
        self.preconditioning_status.as_ref()
    }

    /// Gets a reference to the evse_sleep field.
    ///
    /// # Returns
    ///
    /// *(2.1)* True when EVSE electronics are in sleep mode for this transaction. Default value (when absent) is false.
    pub fn get_evse_sleep(&self) -> Option<&bool> {
        self.evse_sleep.as_ref()
    }

    /// Gets a reference to the transaction_info field.
    ///
    /// # Returns
    ///
    /// The transaction_info field
    pub fn get_transaction_info(&self) -> &TransactionType {
        &self.transaction_info
    }

    /// Gets a reference to the evse field.
    ///
    /// # Returns
    ///
    /// The evse field
    pub fn get_evse(&self) -> Option<&EVSEType> {
        self.evse.as_ref()
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the cost_details field and returns self for builder pattern.
    ///
    /// * `cost_details` - The cost_details field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_cost_details(mut self, cost_details: CostDetailsType) -> Self {
        self.cost_details = Some(cost_details);
        self
    }

    /// Sets the meter_value field and returns self for builder pattern.
    ///
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_meter_value(mut self, meter_value: Vec<MeterValueType>) -> Self {
        self.meter_value = Some(meter_value);
        self
    }

    /// Sets the offline field and returns self for builder pattern.
    ///
    /// * `offline` - Indication that this transaction event happened when the Charging Station was offline. Default = false, meaning: the event occurred when the Charging Station was online.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_offline(mut self, offline: bool) -> Self {
        self.offline = Some(offline);
        self
    }

    /// Sets the number_of_phases_used field and returns self for builder pattern.
    ///
    /// * `number_of_phases_used` - If the Charging Station is able to report the number of phases used, then it SHALL provide it. When omitted the CSMS may be able to determine the number of phases used as follows: + 1: The numberPhases in the currently used ChargingSchedule. + 2: The number of phases provided via device management.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_number_of_phases_used(mut self, number_of_phases_used: i32) -> Self {
        self.number_of_phases_used = Some(number_of_phases_used);
        self
    }

    /// Sets the cable_max_current field and returns self for builder pattern.
    ///
    /// * `cable_max_current` - The maximum current of the connected cable in Ampere (A).
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_cable_max_current(mut self, cable_max_current: i32) -> Self {
        self.cable_max_current = Some(cable_max_current);
        self
    }

    /// Sets the reservation_id field and returns self for builder pattern.
    ///
    /// * `reservation_id` - This contains the Id of the reservation that terminates as a result of this transaction.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_reservation_id(mut self, reservation_id: i32) -> Self {
        self.reservation_id = Some(reservation_id);
        self
    }

    /// Sets the preconditioning_status field and returns self for builder pattern.
    ///
    /// * `preconditioning_status` - The preconditioning_status field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_preconditioning_status(mut self, preconditioning_status: PreconditioningStatusEnumType) -> Self {
        self.preconditioning_status = Some(preconditioning_status);
        self
    }

    /// Sets the evse_sleep field and returns self for builder pattern.
    ///
    /// * `evse_sleep` - *(2.1)* True when EVSE electronics are in sleep mode for this transaction. Default value (when absent) is false.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_sleep(mut self, evse_sleep: bool) -> Self {
        self.evse_sleep = Some(evse_sleep);
        self
    }

    /// Sets the evse field and returns self for builder pattern.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }

    /// Sets the id_token field and returns self for builder pattern.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
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

/// Response body for the TransactionEvent response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    /// When _eventType_ of TransactionEventRequest is Updated, then this value contains the running cost. When _eventType_ of TransactionEventRequest is Ended, then this contains the final total cost of this transaction, including taxes, in the currency configured with the Configuration Variable: Currency. Absence of this value does not imply that the transaction was free. To indicate a free transaction, the CSMS SHALL send a value of 0.00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<Decimal>,

    /// Priority from a business point of view. Default priority is 0, The range is from -9 to 9. Higher values indicate a higher priority. The chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; is temporarily, so it may not be set in the &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt; afterwards. Also the chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; has a higher priority than the one in &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt;.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token_info: Option<IdTokenInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub transaction_limit: Option<TransactionLimitType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub updated_personal_message: Option<MessageContentType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4))]
    #[validate(nested)]
    pub updated_personal_message_extra: Option<Vec<MessageContentType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TransactionEventResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            transaction_limit: None,
            updated_personal_message: None,
            updated_personal_message_extra: None,
            custom_data: None,
        }
    }

    /// Sets the total_cost field.
    ///
    /// * `total_cost` - When _eventType_ of TransactionEventRequest is Updated, then this value contains the running cost. When _eventType_ of TransactionEventRequest is Ended, then this contains the final total cost of this transaction, including taxes, in the currency configured with the Configuration Variable: Currency. Absence of this value does not imply that the transaction was free. To indicate a free transaction, the CSMS SHALL send a value of 0.00.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_total_cost(&mut self, total_cost: Option<Decimal>) -> &mut Self {
        self.total_cost = total_cost;
        self
    }

    /// Sets the charging_priority field.
    ///
    /// * `charging_priority` - Priority from a business point of view. Default priority is 0, The range is from -9 to 9. Higher values indicate a higher priority. The chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; is temporarily, so it may not be set in the &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt; afterwards. Also the chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; has a higher priority than the one in &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt;.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_priority(&mut self, charging_priority: Option<i32>) -> &mut Self {
        self.charging_priority = charging_priority;
        self
    }

    /// Sets the id_token_info field.
    ///
    /// * `id_token_info` - The id_token_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token_info(&mut self, id_token_info: Option<IdTokenInfoType>) -> &mut Self {
        self.id_token_info = id_token_info;
        self
    }

    /// Sets the transaction_limit field.
    ///
    /// * `transaction_limit` - The transaction_limit field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_limit(&mut self, transaction_limit: Option<TransactionLimitType>) -> &mut Self {
        self.transaction_limit = transaction_limit;
        self
    }

    /// Sets the updated_personal_message field.
    ///
    /// * `updated_personal_message` - The updated_personal_message field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_updated_personal_message(&mut self, updated_personal_message: Option<MessageContentType>) -> &mut Self {
        self.updated_personal_message = updated_personal_message;
        self
    }

    /// Sets the updated_personal_message_extra field.
    ///
    /// * `updated_personal_message_extra` - The updated_personal_message_extra field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_updated_personal_message_extra(&mut self, updated_personal_message_extra: Option<Vec<MessageContentType>>) -> &mut Self {
        self.updated_personal_message_extra = updated_personal_message_extra;
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

    /// Gets a reference to the total_cost field.
    ///
    /// # Returns
    ///
    /// When _eventType_ of TransactionEventRequest is Updated, then this value contains the running cost. When _eventType_ of TransactionEventRequest is Ended, then this contains the final total cost of this transaction, including taxes, in the currency configured with the Configuration Variable: Currency. Absence of this value does not imply that the transaction was free. To indicate a free transaction, the CSMS SHALL send a value of 0.00.
    pub fn get_total_cost(&self) -> Option<&Decimal> {
        self.total_cost.as_ref()
    }

    /// Gets a reference to the charging_priority field.
    ///
    /// # Returns
    ///
    /// Priority from a business point of view. Default priority is 0, The range is from -9 to 9. Higher values indicate a higher priority. The chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; is temporarily, so it may not be set in the &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt; afterwards. Also the chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; has a higher priority than the one in &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt;.
    pub fn get_charging_priority(&self) -> Option<&i32> {
        self.charging_priority.as_ref()
    }

    /// Gets a reference to the id_token_info field.
    ///
    /// # Returns
    ///
    /// The id_token_info field
    pub fn get_id_token_info(&self) -> Option<&IdTokenInfoType> {
        self.id_token_info.as_ref()
    }

    /// Gets a reference to the transaction_limit field.
    ///
    /// # Returns
    ///
    /// The transaction_limit field
    pub fn get_transaction_limit(&self) -> Option<&TransactionLimitType> {
        self.transaction_limit.as_ref()
    }

    /// Gets a reference to the updated_personal_message field.
    ///
    /// # Returns
    ///
    /// The updated_personal_message field
    pub fn get_updated_personal_message(&self) -> Option<&MessageContentType> {
        self.updated_personal_message.as_ref()
    }

    /// Gets a reference to the updated_personal_message_extra field.
    ///
    /// # Returns
    ///
    /// The updated_personal_message_extra field
    pub fn get_updated_personal_message_extra(&self) -> Option<&Vec<MessageContentType>> {
        self.updated_personal_message_extra.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the total_cost field and returns self for builder pattern.
    ///
    /// * `total_cost` - When _eventType_ of TransactionEventRequest is Updated, then this value contains the running cost. When _eventType_ of TransactionEventRequest is Ended, then this contains the final total cost of this transaction, including taxes, in the currency configured with the Configuration Variable: Currency. Absence of this value does not imply that the transaction was free. To indicate a free transaction, the CSMS SHALL send a value of 0.00.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_total_cost(mut self, total_cost: Decimal) -> Self {
        self.total_cost = Some(total_cost);
        self
    }

    /// Sets the charging_priority field and returns self for builder pattern.
    ///
    /// * `charging_priority` - Priority from a business point of view. Default priority is 0, The range is from -9 to 9. Higher values indicate a higher priority. The chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; is temporarily, so it may not be set in the &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt; afterwards. Also the chargingPriority in &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; has a higher priority than the one in &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt;.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_priority(mut self, charging_priority: i32) -> Self {
        self.charging_priority = Some(charging_priority);
        self
    }

    /// Sets the id_token_info field and returns self for builder pattern.
    ///
    /// * `id_token_info` - The id_token_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_id_token_info(mut self, id_token_info: IdTokenInfoType) -> Self {
        self.id_token_info = Some(id_token_info);
        self
    }

    /// Sets the transaction_limit field and returns self for builder pattern.
    ///
    /// * `transaction_limit` - The transaction_limit field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_transaction_limit(mut self, transaction_limit: TransactionLimitType) -> Self {
        self.transaction_limit = Some(transaction_limit);
        self
    }

    /// Sets the updated_personal_message field and returns self for builder pattern.
    ///
    /// * `updated_personal_message` - The updated_personal_message field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_updated_personal_message(mut self, updated_personal_message: MessageContentType) -> Self {
        self.updated_personal_message = Some(updated_personal_message);
        self
    }

    /// Sets the updated_personal_message_extra field and returns self for builder pattern.
    ///
    /// * `updated_personal_message_extra` - The updated_personal_message_extra field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_updated_personal_message_extra(mut self, updated_personal_message_extra: Vec<MessageContentType>) -> Self {
        self.updated_personal_message_extra = Some(updated_personal_message_extra);
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
