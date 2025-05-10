use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Tariff assignment to a charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Required. Start date and time of the tariff assignment.
    pub start: String,

    /// Optional. End date and time of the tariff assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}

impl TariffAssignmentType {
    /// Creates a new `TariffAssignmentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier used to identify one or more tariffs
    /// * `start` - Start date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// A new instance of `TariffAssignmentType` with optional fields set to `None`
    pub fn new(id: String, start: String) -> Self {
        Self {
            id,
            start,
            custom_data: None,
            expiry_date: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the expiry date.
    ///
    /// # Arguments
    ///
    /// * `expiry_date` - End date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_expiry_date(mut self, expiry_date: String) -> Self {
        self.expiry_date = Some(expiry_date);
        self
    }

    /// Gets the tariff identifier.
    ///
    /// # Returns
    ///
    /// The unique identifier used to identify one or more tariffs
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the tariff identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier used to identify one or more tariffs
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the start date and time.
    ///
    /// # Returns
    ///
    /// The start date and time of the tariff assignment
    pub fn start(&self) -> &str {
        &self.start
    }

    /// Sets the start date and time.
    ///
    /// # Arguments
    ///
    /// * `start` - Start date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start(&mut self, start: String) -> &mut Self {
        self.start = start;
        self
    }

    /// Gets the expiry date.
    ///
    /// # Returns
    ///
    /// An optional reference to the end date and time of the tariff assignment
    pub fn expiry_date(&self) -> Option<&str> {
        self.expiry_date.as_deref()
    }

    /// Sets the expiry date.
    ///
    /// # Arguments
    ///
    /// * `expiry_date` - End date and time of the tariff assignment, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_expiry_date(&mut self, expiry_date: Option<String>) -> &mut Self {
        self.expiry_date = expiry_date;
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
    /// * `custom_data` - Custom data for this tariff assignment, or None to clear
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
    fn test_new_tariff_assignment() {
        let id = "tariff-123".to_string();
        let start = "2023-01-01T00:00:00Z".to_string();

        let tariff_assignment = TariffAssignmentType::new(id.clone(), start.clone());

        assert_eq!(tariff_assignment.id(), id);
        assert_eq!(tariff_assignment.start(), start);
        assert_eq!(tariff_assignment.expiry_date(), None);
        assert_eq!(tariff_assignment.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "tariff-123".to_string();
        let start = "2023-01-01T00:00:00Z".to_string();
        let expiry_date = "2023-12-31T23:59:59Z".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_assignment = TariffAssignmentType::new(id.clone(), start.clone())
            .with_expiry_date(expiry_date.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_assignment.id(), id);
        assert_eq!(tariff_assignment.start(), start);
        assert_eq!(tariff_assignment.expiry_date(), Some(expiry_date.as_str()));
        assert_eq!(tariff_assignment.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "tariff-123".to_string();
        let start1 = "2023-01-01T00:00:00Z".to_string();
        let id2 = "tariff-456".to_string();
        let start2 = "2023-02-01T00:00:00Z".to_string();
        let expiry_date = "2023-12-31T23:59:59Z".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_assignment = TariffAssignmentType::new(id1, start1);

        tariff_assignment
            .set_id(id2.clone())
            .set_start(start2.clone())
            .set_expiry_date(Some(expiry_date.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_assignment.id(), id2);
        assert_eq!(tariff_assignment.start(), start2);
        assert_eq!(tariff_assignment.expiry_date(), Some(expiry_date.as_str()));
        assert_eq!(tariff_assignment.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_assignment
            .set_expiry_date(None)
            .set_custom_data(None);

        assert_eq!(tariff_assignment.expiry_date(), None);
        assert_eq!(tariff_assignment.custom_data(), None);
    }
}
