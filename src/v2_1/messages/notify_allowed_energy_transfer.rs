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
