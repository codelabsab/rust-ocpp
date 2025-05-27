use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{EnergyTransferModeEnumType, NotifyAllowedEnergyTransferStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyAllowedEnergyTransfer request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferRequest {
    /// The transaction for which the allowed energy transfer is allowed.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Modes of energy transfer that are accepted by CSMS.
    #[validate(length(min = 1))]
    pub allowed_energy_transfer: Vec<EnergyTransferModeEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyAllowedEnergyTransferRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `transaction_id` - The transaction for which the allowed energy transfer is allowed.
    /// * `allowed_energy_transfer` - Modes of energy transfer that are accepted by CSMS.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(transaction_id: String, allowed_energy_transfer: Vec<EnergyTransferModeEnumType>) -> Self {
        Self {
            transaction_id,
            allowed_energy_transfer,
            custom_data: None,
        }
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - The transaction for which the allowed energy transfer is allowed.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Sets the allowed_energy_transfer field.
    ///
    /// * `allowed_energy_transfer` - Modes of energy transfer that are accepted by CSMS.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_allowed_energy_transfer(&mut self, allowed_energy_transfer: Vec<EnergyTransferModeEnumType>) -> &mut Self {
        self.allowed_energy_transfer = allowed_energy_transfer;
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
    /// The transaction for which the allowed energy transfer is allowed.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
    }

    /// Gets a reference to the allowed_energy_transfer field.
    ///
    /// # Returns
    ///
    /// Modes of energy transfer that are accepted by CSMS.
    pub fn get_allowed_energy_transfer(&self) -> &Vec<EnergyTransferModeEnumType> {
        &self.allowed_energy_transfer
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

/// Response body for the NotifyAllowedEnergyTransfer response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferResponse {
    pub status: NotifyAllowedEnergyTransferStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyAllowedEnergyTransferResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: NotifyAllowedEnergyTransferStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: NotifyAllowedEnergyTransferStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &NotifyAllowedEnergyTransferStatusEnumType {
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
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    // Tests for NotifyAllowedEnergyTransferRequest

    #[test]
    fn test_notify_allowed_energy_transfer_request_new() {
        let transaction_id = "txn_123".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id.clone(), allowed_energy_transfer.clone());

        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.allowed_energy_transfer, allowed_energy_transfer);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_with_custom_data() {
        let transaction_id = "txn_456".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::DC];
        let custom_data = create_test_custom_data();
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id.clone(), allowed_energy_transfer.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.allowed_energy_transfer, allowed_energy_transfer);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_setters() {
        let transaction_id1 = "txn_111".to_string();
        let transaction_id2 = "txn_222".to_string();
        let allowed_energy_transfer1 = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let allowed_energy_transfer2 = vec![EnergyTransferModeEnumType::DC, EnergyTransferModeEnumType::ACThreePhase];
        let custom_data = create_test_custom_data();

        let mut request = NotifyAllowedEnergyTransferRequest::new(transaction_id1, allowed_energy_transfer1);
        request.set_transaction_id(transaction_id2.clone());
        request.set_allowed_energy_transfer(allowed_energy_transfer2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.transaction_id, transaction_id2);
        assert_eq!(request.allowed_energy_transfer, allowed_energy_transfer2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_getters() {
        let transaction_id = "txn_789".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACBPT];
        let custom_data = create_test_custom_data();
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id.clone(), allowed_energy_transfer.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_transaction_id(), &transaction_id);
        assert_eq!(request.get_allowed_energy_transfer(), &allowed_energy_transfer);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_serialization() {
        let transaction_id = "txn_serialization".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id, allowed_energy_transfer);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NotifyAllowedEnergyTransferRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_validation() {
        let transaction_id = "txn_valid".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id, allowed_energy_transfer);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_validation_long_transaction_id() {
        let long_transaction_id = "a".repeat(37); // Exceeds max length of 36
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let mut request = NotifyAllowedEnergyTransferRequest::new("valid".to_string(), allowed_energy_transfer);
        request.set_transaction_id(long_transaction_id);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_validation_max_transaction_id() {
        let max_transaction_id = "a".repeat(36); // Exactly at max length
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let request = NotifyAllowedEnergyTransferRequest::new(max_transaction_id, allowed_energy_transfer);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_validation_empty_allowed_energy_transfer() {
        let transaction_id = "txn_empty".to_string();
        let mut request = NotifyAllowedEnergyTransferRequest::new(transaction_id, vec![EnergyTransferModeEnumType::ACSinglePhase]);
        request.set_allowed_energy_transfer(vec![]); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_multiple_energy_transfer_modes() {
        let transaction_id = "txn_multiple".to_string();
        let allowed_energy_transfer = vec![
            EnergyTransferModeEnumType::ACSinglePhase,
            EnergyTransferModeEnumType::ACTwoPhase,
            EnergyTransferModeEnumType::ACThreePhase,
            EnergyTransferModeEnumType::DC,
        ];
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id, allowed_energy_transfer.clone());

        assert_eq!(request.allowed_energy_transfer.len(), 4);
        assert_eq!(request.allowed_energy_transfer, allowed_energy_transfer);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_all_energy_transfer_modes() {
        let transaction_id = "txn_all_modes".to_string();
        let energy_transfer_modes = vec![
            EnergyTransferModeEnumType::ACSinglePhase,
            EnergyTransferModeEnumType::ACTwoPhase,
            EnergyTransferModeEnumType::ACThreePhase,
            EnergyTransferModeEnumType::DC,
            EnergyTransferModeEnumType::ACBPT,
            EnergyTransferModeEnumType::ACBPTDER,
            EnergyTransferModeEnumType::ACDER,
            EnergyTransferModeEnumType::DCBPT,
            EnergyTransferModeEnumType::DCACDP,
            EnergyTransferModeEnumType::DCACDPBPT,
            EnergyTransferModeEnumType::WPT,
        ];

        for mode in energy_transfer_modes {
            let request = NotifyAllowedEnergyTransferRequest::new(transaction_id.clone(), vec![mode.clone()]);
            assert_eq!(request.allowed_energy_transfer, vec![mode]);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_allowed_energy_transfer_request_json_round_trip() {
        let transaction_id = "txn_round_trip".to_string();
        let allowed_energy_transfer = vec![EnergyTransferModeEnumType::DC, EnergyTransferModeEnumType::ACThreePhase];
        let custom_data = create_test_custom_data();
        let request = NotifyAllowedEnergyTransferRequest::new(transaction_id, allowed_energy_transfer)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NotifyAllowedEnergyTransferRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for NotifyAllowedEnergyTransferResponse

    #[test]
    fn test_notify_allowed_energy_transfer_response_new() {
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted);

        assert_eq!(response.status, NotifyAllowedEnergyTransferStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, NotifyAllowedEnergyTransferStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, NotifyAllowedEnergyTransferStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted);
        response.set_status(NotifyAllowedEnergyTransferStatusEnumType::Rejected);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, NotifyAllowedEnergyTransferStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &NotifyAllowedEnergyTransferStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_serialization() {
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: NotifyAllowedEnergyTransferResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_validation() {
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_all_status_types() {
        let statuses = vec![
            NotifyAllowedEnergyTransferStatusEnumType::Accepted,
            NotifyAllowedEnergyTransferStatusEnumType::Rejected,
        ];

        for status in statuses {
            let response = NotifyAllowedEnergyTransferResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_allowed_energy_transfer_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = NotifyAllowedEnergyTransferResponse::new(NotifyAllowedEnergyTransferStatusEnumType::Rejected)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: NotifyAllowedEnergyTransferResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}