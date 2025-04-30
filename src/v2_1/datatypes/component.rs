use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, evse::EVSEType};

/// A physical or logical component
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Specifies the EVSE when component is located at EVSE level, also specifies the connector when component is located at Connector level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,

    /// Name of the component. Name should be taken from the list of standardized component names whenever possible.
    /// Case Insensitive. strongly advised to use Camel Case.
    #[validate(length(max = 50))]
    pub name: String,

    /// Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub instance: Option<String>,
}

impl ComponentType {
    /// Creates a new `ComponentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the component
    ///
    /// # Returns
    ///
    /// A new instance of `ComponentType` with optional fields set to `None`
    pub fn new(name: String) -> Self {
        Self {
            name,
            custom_data: None,
            evse: None,
            instance: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the EVSE.
    ///
    /// # Arguments
    ///
    /// * `evse` - EVSE for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }

    /// Sets the instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Instance name for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    /// Gets the name.
    ///
    /// # Returns
    ///
    /// A reference to the name of the component
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
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
    /// * `custom_data` - Custom data for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the EVSE.
    ///
    /// # Returns
    ///
    /// An optional reference to the EVSE
    pub fn evse(&self) -> Option<&EVSEType> {
        self.evse.as_ref()
    }

    /// Sets the EVSE.
    ///
    /// # Arguments
    ///
    /// * `evse` - EVSE for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse(&mut self, evse: Option<EVSEType>) -> &mut Self {
        self.evse = evse;
        self
    }

    /// Gets the instance.
    ///
    /// # Returns
    ///
    /// An optional reference to the instance name
    pub fn instance(&self) -> Option<&str> {
        self.instance.as_deref()
    }

    /// Sets the instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Instance name for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_instance(&mut self, instance: Option<String>) -> &mut Self {
        self.instance = instance;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_component() {
        let component = ComponentType::new("Connector".to_string());

        assert_eq!(component.name(), "Connector");
        assert_eq!(component.custom_data(), None);
        assert_eq!(component.evse(), None);
        assert_eq!(component.instance(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let evse = EVSEType {
            id: 1,
            connector_id: Some(2),
            custom_data: None,
        };

        let component = ComponentType::new("Connector".to_string())
            .with_custom_data(custom_data.clone())
            .with_evse(evse.clone())
            .with_instance("Main".to_string());

        assert_eq!(component.name(), "Connector");
        assert_eq!(component.custom_data(), Some(&custom_data));
        assert_eq!(component.evse(), Some(&evse));
        assert_eq!(component.instance(), Some("Main"));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let evse = EVSEType {
            id: 1,
            connector_id: Some(2),
            custom_data: None,
        };

        let mut component = ComponentType::new("Connector".to_string());

        component
            .set_name("Meter".to_string())
            .set_custom_data(Some(custom_data.clone()))
            .set_evse(Some(evse.clone()))
            .set_instance(Some("Secondary".to_string()));

        assert_eq!(component.name(), "Meter");
        assert_eq!(component.custom_data(), Some(&custom_data));
        assert_eq!(component.evse(), Some(&evse));
        assert_eq!(component.instance(), Some("Secondary"));

        // Test clearing optional fields
        component
            .set_custom_data(None)
            .set_evse(None)
            .set_instance(None);

        assert_eq!(component.custom_data(), None);
        assert_eq!(component.evse(), None);
        assert_eq!(component.instance(), None);
    }
}
