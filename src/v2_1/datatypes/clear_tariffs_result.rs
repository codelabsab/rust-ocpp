use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType};

/// Status of operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TariffClearStatusEnumType {
    Accepted,
    Rejected,
    NoTariff,
}

/// Result of clearing a tariff.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResultType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Id of tariff for which _status_ is reported. If no tariffs were found, then this field is absent, and _status_ will be `NoTariff`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Status indicating whether the tariff was cleared.
    pub status: TariffClearStatusEnumType,
}

impl ClearTariffsResultType {
    /// Creates a new `ClearTariffsResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the tariff was cleared
    ///
    /// # Returns
    ///
    /// A new instance of `ClearTariffsResultType` with optional fields set to `None`
    pub fn new(status: TariffClearStatusEnumType) -> Self {
        Self {
            status,
            tariff_id: None,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Id of tariff for which status is reported
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tariff_id(mut self, tariff_id: String) -> Self {
        self.tariff_id = Some(tariff_id);
        self
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear tariffs result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// The status indicating whether the tariff was cleared
    pub fn status(&self) -> &TariffClearStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the tariff was cleared
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: TariffClearStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the tariff ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the ID of tariff for which status is reported
    pub fn tariff_id(&self) -> Option<&str> {
        self.tariff_id.as_deref()
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Id of tariff for which status is reported, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: Option<String>) -> &mut Self {
        self.tariff_id = tariff_id;
        self
    }

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the element providing more information about the status
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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
    /// * `custom_data` - Custom data for this clear tariffs result, or None to clear
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
    fn test_new_clear_tariffs_result() {
        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted);

        assert_eq!(result.status(), &TariffClearStatusEnumType::Accepted);
        assert_eq!(result.tariff_id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Rejected)
            .with_tariff_id("tariff-123".to_string())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(result.status(), &TariffClearStatusEnumType::Rejected);
        assert_eq!(result.tariff_id(), Some("tariff-123"));
        assert_eq!(result.status_info().unwrap().reason_code(), "SomeReason");
        assert_eq!(result.status_info().unwrap().additional_info(), Some("Additional details"));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted);

        result
            .set_status(TariffClearStatusEnumType::NoTariff)
            .set_tariff_id(Some("tariff-456".to_string()))
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.status(), &TariffClearStatusEnumType::NoTariff);
        assert_eq!(result.tariff_id(), Some("tariff-456"));
        assert_eq!(result.status_info().unwrap().reason_code(), "SomeReason");
        assert_eq!(result.status_info().unwrap().additional_info(), Some("Additional details"));
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_tariff_id(None)
            .set_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.tariff_id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }
}
