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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::{IdTokenType, StatusInfoType};
    use crate::v2_1::enumerations::ReserveNowStatusEnumType;
    use chrono::{DateTime, Utc};
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

    fn create_test_expiry_date_time() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z").unwrap().with_timezone(&Utc)
    }

    #[test]
    fn test_reserve_now_request_new() {
        let id_token = create_test_id_token();
        let expiry_date_time = create_test_expiry_date_time();
        let request = ReserveNowRequest::new(
            123,
            expiry_date_time,
            id_token.clone()
        );

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.expiry_date_time, expiry_date_time);
        assert_eq!(request.id, 123);
        assert!(request.connector_type.is_none());
        assert!(request.evse_id.is_none());
        assert!(request.group_id_token.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_reserve_now_request_serialization() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "Local".to_string());
        let expiry_date_time = create_test_expiry_date_time();

        let request = ReserveNowRequest::new(
            456,
            expiry_date_time,
            id_token.clone()
        )
        .with_connector_type("cCCS1".to_string())
        .with_evse_id(2)
        .with_group_id_token(group_id_token.clone())
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ReserveNowRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.id_token, id_token);
        assert_eq!(deserialized.expiry_date_time, expiry_date_time);
        assert_eq!(deserialized.id, 456);
        assert_eq!(deserialized.connector_type, Some("cCCS1".to_string()));
        assert_eq!(deserialized.evse_id, Some(2));
        assert_eq!(deserialized.group_id_token, Some(group_id_token));
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_reserve_now_request_validation() {
        let id_token = create_test_id_token();
        let expiry_date_time = create_test_expiry_date_time();

        // Valid request
        let valid_request = ReserveNowRequest::new(
            0,
            expiry_date_time,
            id_token.clone()
        );
        assert!(valid_request.validate().is_ok());

        // Invalid id (negative)
        let invalid_request = ReserveNowRequest {
            connector_type: None,
            evse_id: None,
            group_id_token: None,
            id_token: id_token.clone(),
            expiry_date_time,
            id: -1,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());

        // Invalid evse_id (negative)
        let invalid_evse_request = ReserveNowRequest {
            connector_type: None,
            evse_id: Some(-1),
            group_id_token: None,
            id_token: id_token,
            expiry_date_time,
            id: 1,
            custom_data: None,
        };
        assert!(invalid_evse_request.validate().is_err());
    }

    #[test]
    fn test_reserve_now_request_builder_pattern() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "KeyCode".to_string());
        let expiry_date_time = create_test_expiry_date_time();
        let custom_data = create_test_custom_data();

        let request = ReserveNowRequest::new(
            789,
            expiry_date_time,
            id_token.clone()
        )
        .with_connector_type("cCCS2".to_string())
        .with_evse_id(3)
        .with_group_id_token(group_id_token.clone())
        .with_custom_data(custom_data.clone());

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.expiry_date_time, expiry_date_time);
        assert_eq!(request.id, 789);
        assert_eq!(request.connector_type, Some("cCCS2".to_string()));
        assert_eq!(request.evse_id, Some(3));
        assert_eq!(request.group_id_token, Some(group_id_token));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reserve_now_response_new() {
        let response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted);

        assert_eq!(response.status, ReserveNowStatusEnumType::Accepted);
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_reserve_now_response_serialization() {
        let response = ReserveNowResponse::new(ReserveNowStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ReserveNowResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.status, ReserveNowStatusEnumType::Rejected);
        assert!(deserialized.status_info.is_some());
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_reserve_now_response_validation() {
        let valid_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted);
        assert!(valid_response.validate().is_ok());

        let response_with_all_fields = ReserveNowResponse::new(ReserveNowStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());
        assert!(response_with_all_fields.validate().is_ok());
    }

    #[test]
    fn test_reserve_now_response_builder_pattern() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, ReserveNowStatusEnumType::Accepted);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reserve_now_response_setters_getters() {
        let mut response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted);
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        // Test setters
        response.set_status(ReserveNowStatusEnumType::Rejected);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*response.get_status(), ReserveNowStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_reserve_now_response_enum_variants() {
        let accepted_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted);
        assert_eq!(accepted_response.status, ReserveNowStatusEnumType::Accepted);

        let rejected_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Rejected);
        assert_eq!(rejected_response.status, ReserveNowStatusEnumType::Rejected);

        let faulted_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Faulted);
        assert_eq!(faulted_response.status, ReserveNowStatusEnumType::Faulted);

        let occupied_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Occupied);
        assert_eq!(occupied_response.status, ReserveNowStatusEnumType::Occupied);

        let unavailable_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Unavailable);
        assert_eq!(unavailable_response.status, ReserveNowStatusEnumType::Unavailable);
    }

    #[test]
    fn test_reserve_now_request_connector_types() {
        let id_token = create_test_id_token();
        let expiry_date_time = create_test_expiry_date_time();

        // Test different connector types as strings
        let connector_types = vec![
            "cCCS1",
            "cCCS2",
            "cG105",
            "cTesla",
            "cType1",
            "cType2",
            "sType2",
            "sType3",
            "Other1PhMax16A",
            "wInductive",
            "Custom",
        ];

        for connector_type in connector_types {
            let request = ReserveNowRequest::new(
                1,
                expiry_date_time,
                id_token.clone()
            ).with_connector_type(connector_type.to_string());
            assert!(request.validate().is_ok());
            assert_eq!(request.connector_type, Some(connector_type.to_string()));
        }
    }

    #[test]
    fn test_reserve_now_json_round_trip() {
        let id_token = create_test_id_token();
        let group_id_token = IdTokenType::new("group_token".to_string(), "eMAID".to_string());
        let expiry_date_time = create_test_expiry_date_time();

        let original_request = ReserveNowRequest::new(
            12345,
            expiry_date_time,
            id_token
        )
        .with_connector_type("cCCS2".to_string())
        .with_evse_id(10)
        .with_group_id_token(group_id_token)
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: ReserveNowRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = ReserveNowResponse::new(ReserveNowStatusEnumType::Accepted)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: ReserveNowResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_reserve_now_request_edge_cases() {
        let id_token = create_test_id_token();
        let expiry_date_time = create_test_expiry_date_time();

        // Test minimum valid id
        let min_request = ReserveNowRequest::new(0, expiry_date_time, id_token.clone());
        assert!(min_request.validate().is_ok());

        // Test large id
        let large_request = ReserveNowRequest::new(i32::MAX, expiry_date_time, id_token.clone());
        assert!(large_request.validate().is_ok());

        // Test minimum valid evse_id
        let min_evse_request = ReserveNowRequest::new(1, expiry_date_time, id_token)
            .with_evse_id(0);
        assert!(min_evse_request.validate().is_ok());
    }

    #[test]
    fn test_reserve_now_request_datetime_handling() {
        let id_token = create_test_id_token();

        // Test different datetime formats
        let datetime1 = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let datetime2 = DateTime::parse_from_rfc3339("2025-12-31T23:59:59.999Z").unwrap().with_timezone(&Utc);

        let request1 = ReserveNowRequest::new(1, datetime1, id_token.clone());
        let request2 = ReserveNowRequest::new(2, datetime2, id_token);

        assert!(request1.validate().is_ok());
        assert!(request2.validate().is_ok());
        assert_eq!(request1.expiry_date_time, datetime1);
        assert_eq!(request2.expiry_date_time, datetime2);
    }
}