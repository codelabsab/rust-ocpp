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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::{ChargingProfileType, ChargingScheduleType, IdTokenType, StatusInfoType};
    use crate::v2_1::enumerations::{ChargingProfileKindEnumType, ChargingProfilePurposeEnumType, ChargingRateUnitEnumType, RequestStartStopStatusEnumType};
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_id_token() -> IdTokenType {
        IdTokenType::new("test_token".to_string(), "Central".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_charging_profile() -> ChargingProfileType {
        let schedule = ChargingScheduleType::new(
            1,
            ChargingRateUnitEnumType::W,
            vec![]
        );
        ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::TxProfile,
            ChargingProfileKindEnumType::Absolute,
            vec![schedule]
        )
    }

    #[test]
    fn test_request_start_transaction_request_new() {
        let id_token = create_test_id_token();
        let request = RequestStartTransactionRequest::new(id_token.clone(), 123);

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.remote_start_id, 123);
        assert!(request.evse_id.is_none());
        assert!(request.group_id_token.is_none());
        assert!(request.charging_profile.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_request_start_transaction_request_serialization() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "Local".to_string());
        let charging_profile = create_test_charging_profile();

        let request = RequestStartTransactionRequest::new(id_token.clone(), 456)
            .with_evse_id(2)
            .with_group_id_token(group_id_token.clone())
            .with_charging_profile(charging_profile.clone())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: RequestStartTransactionRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.id_token, id_token);
        assert_eq!(deserialized.remote_start_id, 456);
        assert_eq!(deserialized.evse_id, Some(2));
        assert_eq!(deserialized.group_id_token, Some(group_id_token));
        assert_eq!(deserialized.charging_profile, Some(charging_profile));
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_start_transaction_request_validation() {
        let id_token = create_test_id_token();

        // Valid request
        let valid_request = RequestStartTransactionRequest::new(id_token.clone(), 0);
        assert!(valid_request.validate().is_ok());

        // Invalid remote_start_id (negative)
        let invalid_request = RequestStartTransactionRequest {
            evse_id: None,
            group_id_token: None,
            id_token: id_token.clone(),
            remote_start_id: -1,
            charging_profile: None,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());

        // Invalid evse_id (zero - should be > 0 if present)
        let invalid_evse_request = RequestStartTransactionRequest {
            evse_id: Some(0),
            group_id_token: None,
            id_token: id_token.clone(),
            remote_start_id: 1,
            charging_profile: None,
            custom_data: None,
        };
        assert!(invalid_evse_request.validate().is_err());
    }

    #[test]
    fn test_request_start_transaction_request_builder_pattern() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "KeyCode".to_string());
        let charging_profile = create_test_charging_profile();
        let custom_data = create_test_custom_data();

        let request = RequestStartTransactionRequest::new(id_token.clone(), 789)
            .with_evse_id(3)
            .with_group_id_token(group_id_token.clone())
            .with_charging_profile(charging_profile.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.remote_start_id, 789);
        assert_eq!(request.evse_id, Some(3));
        assert_eq!(request.group_id_token, Some(group_id_token));
        assert_eq!(request.charging_profile, Some(charging_profile));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_start_transaction_request_setters_getters() {
        let id_token = create_test_id_token();
        let mut request = RequestStartTransactionRequest::new(id_token.clone(), 100);
        let new_id_token = IdTokenType::new("new_token".to_string(), "ISO14443".to_string());
        let group_id_token = IdTokenType::new("group_token".to_string(), "MacAddress".to_string());
        let charging_profile = create_test_charging_profile();
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_evse_id(Some(5));
        request.set_group_id_token(Some(group_id_token.clone()));
        request.set_id_token(new_id_token.clone());
        request.set_remote_start_id(200);
        request.set_charging_profile(Some(charging_profile.clone()));
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(request.get_evse_id(), Some(&5));
        assert_eq!(request.get_group_id_token(), Some(&group_id_token));
        assert_eq!(request.get_id_token(), &new_id_token);
        assert_eq!(*request.get_remote_start_id(), 200);
        assert_eq!(request.get_charging_profile(), Some(&charging_profile));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_start_transaction_response_new() {
        let response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);

        assert_eq!(response.status, RequestStartStopStatusEnumType::Accepted);
        assert!(response.status_info.is_none());
        assert!(response.transaction_id.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_request_start_transaction_response_serialization() {
        let response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_transaction_id("tx-12345".to_string())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: RequestStartTransactionResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.status, RequestStartStopStatusEnumType::Rejected);
        assert!(deserialized.status_info.is_some());
        assert_eq!(deserialized.transaction_id, Some("tx-12345".to_string()));
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_start_transaction_response_validation() {
        let valid_response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        assert!(valid_response.validate().is_ok());

        let response_with_all_fields = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_transaction_id("tx-67890".to_string())
            .with_custom_data(create_test_custom_data());
        assert!(response_with_all_fields.validate().is_ok());

        // Test transaction_id length validation (max 36 characters)
        let long_transaction_id = "a".repeat(37);
        let invalid_response = RequestStartTransactionResponse {
            status: RequestStartStopStatusEnumType::Accepted,
            status_info: None,
            transaction_id: Some(long_transaction_id),
            custom_data: None,
        };
        assert!(invalid_response.validate().is_err());
    }

    #[test]
    fn test_request_start_transaction_response_builder_pattern() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_transaction_id("tx-abc123".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, RequestStartStopStatusEnumType::Accepted);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.transaction_id, Some("tx-abc123".to_string()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_start_transaction_response_setters_getters() {
        let mut response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        // Test setters
        response.set_status(RequestStartStopStatusEnumType::Rejected);
        response.set_status_info(Some(status_info.clone()));
        response.set_transaction_id(Some("tx-xyz789".to_string()));
        response.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*response.get_status(), RequestStartStopStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_transaction_id(), Some(&"tx-xyz789".to_string()));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_start_transaction_response_enum_variants() {
        let accepted_response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        assert_eq!(accepted_response.status, RequestStartStopStatusEnumType::Accepted);

        let rejected_response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Rejected);
        assert_eq!(rejected_response.status, RequestStartStopStatusEnumType::Rejected);
    }

    #[test]
    fn test_request_start_transaction_request_edge_cases() {
        let id_token = create_test_id_token();

        // Test minimum valid remote_start_id
        let min_request = RequestStartTransactionRequest::new(id_token.clone(), 0);
        assert!(min_request.validate().is_ok());

        // Test large remote_start_id
        let large_request = RequestStartTransactionRequest::new(id_token.clone(), i32::MAX);
        assert!(large_request.validate().is_ok());

        // Test minimum valid evse_id
        let min_evse_request = RequestStartTransactionRequest::new(id_token, 1)
            .with_evse_id(1);
        assert!(min_evse_request.validate().is_ok());
    }

    #[test]
    fn test_request_start_transaction_json_round_trip() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "eMAID".to_string());
        let charging_profile = create_test_charging_profile();

        let original_request = RequestStartTransactionRequest::new(id_token, 12345)
            .with_evse_id(10)
            .with_group_id_token(group_id_token)
            .with_charging_profile(charging_profile)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: RequestStartTransactionRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_status_info(create_test_status_info())
            .with_transaction_id("tx-round-trip".to_string())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: RequestStartTransactionResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_request_start_transaction_id_token_types() {
        // Test different IdToken types
        let token_types = vec![
            "Central",
            "DirectPayment",
            "eMAID",
            "EVCCID",
            "ISO14443",
            "ISO15693",
            "KeyCode",
            "Local",
            "MacAddress",
            "NoAuthorization",
            "VIN",
        ];

        for token_type in token_types {
            let id_token = IdTokenType::new("test_token".to_string(), token_type.to_string());
            let request = RequestStartTransactionRequest::new(id_token.clone(), 1);
            assert!(request.validate().is_ok());
            assert_eq!(request.id_token, id_token);
        }
    }

    #[test]
    fn test_request_start_transaction_transaction_id_edge_cases() {
        // Test maximum valid transaction_id length (36 characters)
        let max_transaction_id = "a".repeat(36);
        let response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_transaction_id(max_transaction_id.clone());
        assert!(response.validate().is_ok());
        assert_eq!(response.transaction_id, Some(max_transaction_id));

        // Test empty transaction_id
        let empty_response = RequestStartTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_transaction_id("".to_string());
        assert!(empty_response.validate().is_ok());
    }
}