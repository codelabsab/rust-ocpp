use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffType};
use crate::v2_1::enumerations::TariffChangeStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ChangeTransactionTariff request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffRequest {
    #[validate(nested)]
    pub tariff: TariffType,

    /// Transaction id for new tariff.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChangeTransactionTariffRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `tariff` - The tariff field
    /// * `transaction_id` - Transaction id for new tariff.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(tariff: TariffType, transaction_id: String) -> Self {
        Self {
            tariff,
            transaction_id,
            custom_data: None,
        }
    }

    /// Sets the tariff field.
    ///
    /// * `tariff` - The tariff field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff(&mut self, tariff: TariffType) -> &mut Self {
        self.tariff = tariff;
        self
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - Transaction id for new tariff.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
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

    /// Gets a reference to the tariff field.
    ///
    /// # Returns
    ///
    /// The tariff field
    pub fn get_tariff(&self) -> &TariffType {
        &self.tariff
    }

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// Transaction id for new tariff.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
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

/// Response body for the ChangeTransactionTariff response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffResponse {
    pub status: TariffChangeStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChangeTransactionTariffResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: TariffChangeStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: TariffChangeStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &TariffChangeStatusEnumType {
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

    fn create_test_tariff() -> TariffType {
        TariffType::new("test-tariff-123".to_string(), "USD".to_string())
    }

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    #[test]
    fn test_change_transaction_tariff_request_new() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();

        let request = ChangeTransactionTariffRequest::new(tariff.clone(), transaction_id.clone());

        assert_eq!(request.tariff, tariff);
        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_change_transaction_tariff_request_with_custom_data() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();
        let custom_data = create_test_custom_data();

        let request = ChangeTransactionTariffRequest::new(tariff.clone(), transaction_id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.tariff, tariff);
        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_request_setters() {
        let mut request = ChangeTransactionTariffRequest::new(
            create_test_tariff(),
            "tx-12345".to_string()
        );

        let new_tariff = TariffType::new("new-tariff-456".to_string(), "EUR".to_string());
        let new_transaction_id = "tx-67890".to_string();
        let custom_data = create_test_custom_data();

        request.set_tariff(new_tariff.clone())
               .set_transaction_id(new_transaction_id.clone())
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.tariff, new_tariff);
        assert_eq!(request.transaction_id, new_transaction_id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_request_getters() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();
        let custom_data = create_test_custom_data();

        let request = ChangeTransactionTariffRequest::new(tariff.clone(), transaction_id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_tariff(), &tariff);
        assert_eq!(request.get_transaction_id(), &transaction_id);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_request_serialization() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();

        let request = ChangeTransactionTariffRequest::new(tariff, transaction_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeTransactionTariffRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_change_transaction_tariff_request_validation_transaction_id_too_long() {
        let tariff = create_test_tariff();
        let long_transaction_id = "a".repeat(37); // Max is 36

        let request = ChangeTransactionTariffRequest::new(tariff, long_transaction_id);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_change_transaction_tariff_request_validation_valid() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();

        let request = ChangeTransactionTariffRequest::new(tariff, transaction_id);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_change_transaction_tariff_response_new() {
        let status = TariffChangeStatusEnumType::Accepted;

        let response = ChangeTransactionTariffResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_change_transaction_tariff_response_with_status_info() {
        let status = TariffChangeStatusEnumType::Rejected;
        let status_info = create_test_status_info();

        let response = ChangeTransactionTariffResponse::new(status.clone())
            .with_status_info(status_info.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_change_transaction_tariff_response_with_custom_data() {
        let status = TariffChangeStatusEnumType::Accepted;
        let custom_data = create_test_custom_data();

        let response = ChangeTransactionTariffResponse::new(status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_response_setters() {
        let mut response = ChangeTransactionTariffResponse::new(TariffChangeStatusEnumType::Accepted);

        let new_status = TariffChangeStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        response.set_status(new_status.clone())
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, new_status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_response_getters() {
        let status = TariffChangeStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = ChangeTransactionTariffResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_response_serialization() {
        let status = TariffChangeStatusEnumType::Accepted;

        let response = ChangeTransactionTariffResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ChangeTransactionTariffResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_change_transaction_tariff_response_all_status_variants() {
        let statuses = vec![
            TariffChangeStatusEnumType::Accepted,
            TariffChangeStatusEnumType::Rejected,
            TariffChangeStatusEnumType::InvalidId,
        ];

        for status in statuses {
            let response = ChangeTransactionTariffResponse::new(status.clone());
            assert_eq!(response.status, status);

            // Test serialization for each variant
            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: ChangeTransactionTariffResponse =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_change_transaction_tariff_response_validation() {
        let status = TariffChangeStatusEnumType::Accepted;
        let response = ChangeTransactionTariffResponse::new(status);

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_change_transaction_tariff_request_builder_pattern() {
        let tariff = create_test_tariff();
        let transaction_id = "tx-12345".to_string();
        let custom_data = create_test_custom_data();

        let request = ChangeTransactionTariffRequest::new(tariff.clone(), transaction_id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.tariff, tariff);
        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_change_transaction_tariff_response_builder_pattern() {
        let status = TariffChangeStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = ChangeTransactionTariffResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }
}