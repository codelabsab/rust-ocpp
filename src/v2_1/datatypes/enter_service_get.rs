use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, enter_service::EnterServiceType};

/// Type for getting EnterService DER control function parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The EnterService parameters.
    pub enter_service: EnterServiceType,

    /// Id of setting.
    #[validate(length(max = 36))]
    pub id: String,
}

impl EnterServiceGetType {
    /// Creates a new `EnterServiceGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `enter_service` - The EnterService parameters
    /// * `id` - Id of setting
    ///
    /// # Returns
    ///
    /// A new instance of `EnterServiceGetType` with optional fields set to `None`
    pub fn new(enter_service: EnterServiceType, id: String) -> Self {
        Self {
            enter_service,
            id,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this enter service get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EnterService parameters.
    ///
    /// # Returns
    ///
    /// A reference to the EnterService parameters
    pub fn enter_service(&self) -> &EnterServiceType {
        &self.enter_service
    }

    /// Sets the EnterService parameters.
    ///
    /// # Arguments
    ///
    /// * `enter_service` - The EnterService parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_enter_service(&mut self, enter_service: EnterServiceType) -> &mut Self {
        self.enter_service = enter_service;
        self
    }

    /// Gets the ID of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the ID of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
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
    /// * `custom_data` - Custom data for this enter service get, or None to clear
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
    fn test_new_enter_service_get() {
        let enter_service = EnterServiceType::new(1, 240.0, 220.0, 60.5, 59.5, 5.0, 2.0, 10.0);
        let id = "setting1".to_string();

        let enter_service_get = EnterServiceGetType::new(enter_service.clone(), id.clone());

        assert_eq!(enter_service_get.enter_service(), &enter_service);
        assert_eq!(enter_service_get.id(), id);
        assert_eq!(enter_service_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let enter_service = EnterServiceType::new(1, 240.0, 220.0, 60.5, 59.5, 5.0, 2.0, 10.0);
        let id = "setting1".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let enter_service_get = EnterServiceGetType::new(enter_service.clone(), id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(enter_service_get.enter_service(), &enter_service);
        assert_eq!(enter_service_get.id(), id);
        assert_eq!(enter_service_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let enter_service1 = EnterServiceType::new(1, 240.0, 220.0, 60.5, 59.5, 5.0, 2.0, 10.0);
        let enter_service2 = EnterServiceType::new(2, 245.0, 215.0, 61.0, 59.0, 6.0, 3.0, 12.0);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut enter_service_get = EnterServiceGetType::new(enter_service1.clone(), id1.clone());

        enter_service_get
            .set_enter_service(enter_service2.clone())
            .set_id(id2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(enter_service_get.enter_service(), &enter_service2);
        assert_eq!(enter_service_get.id(), id2);
        assert_eq!(enter_service_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        enter_service_get.set_custom_data(None);
        assert_eq!(enter_service_get.custom_data(), None);
    }
}
