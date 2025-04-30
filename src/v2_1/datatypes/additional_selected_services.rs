use crate::v2_1::datatypes::{custom_data::CustomDataType, rational_number::RationalNumberType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Part of ISO 15118-20 price schedule.
///
/// This type represents additional services that can be selected as part of a charging session,
/// including the service name and associated fee.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalSelectedServicesType {
    /// Service fee
    ///
    /// The fee associated with this additional service, represented as a rational number.
    pub service_fee: RationalNumberType,

    /// Human-readable string to identify this service.
    ///
    /// A descriptive name for the service that can be displayed to users.
    pub service_name: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl AdditionalSelectedServicesType {
    /// Creates a new `AdditionalSelectedServicesType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `service_fee` - The fee associated with this additional service
    /// * `service_name` - Human-readable string to identify this service
    ///
    /// # Returns
    ///
    /// A new instance of `AdditionalSelectedServicesType` with optional fields set to `None`
    pub fn new(service_fee: RationalNumberType, service_name: String) -> Self {
        Self {
            service_fee,
            service_name,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this additional selected service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the service fee.
    ///
    /// # Returns
    ///
    /// A reference to the service fee as a rational number
    pub fn service_fee(&self) -> &RationalNumberType {
        &self.service_fee
    }

    /// Sets the service fee.
    ///
    /// # Arguments
    ///
    /// * `service_fee` - The fee associated with this additional service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_service_fee(&mut self, service_fee: RationalNumberType) -> &mut Self {
        self.service_fee = service_fee;
        self
    }

    /// Gets the service name.
    ///
    /// # Returns
    ///
    /// The service name as a string
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Sets the service name.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Human-readable string to identify this service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_service_name(&mut self, service_name: String) -> &mut Self {
        self.service_name = service_name;
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
    /// * `custom_data` - Custom data for this additional selected service, or None to clear
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
    fn test_new_additional_selected_services() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let services = AdditionalSelectedServicesType::new(
            service_fee.clone(),
            "Premium Charging".to_string(),
        );

        assert_eq!(services.service_fee(), &service_fee);
        assert_eq!(services.service_name(), "Premium Charging");
        assert_eq!(services.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let services = AdditionalSelectedServicesType::new(
            service_fee.clone(),
            "Premium Charging".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(services.service_fee(), &service_fee);
        assert_eq!(services.service_name(), "Premium Charging");
        assert_eq!(services.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let service_fee1 = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let service_fee2 = RationalNumberType {
            exponent: 2,
            value: 2495,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut services = AdditionalSelectedServicesType::new(
            service_fee1.clone(),
            "Premium Charging".to_string(),
        );

        services
            .set_service_fee(service_fee2.clone())
            .set_service_name("Ultra Premium Charging".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(services.service_fee(), &service_fee2);
        assert_eq!(services.service_name(), "Ultra Premium Charging");
        assert_eq!(services.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        services.set_custom_data(None);
        assert_eq!(services.custom_data(), None);
    }
}
