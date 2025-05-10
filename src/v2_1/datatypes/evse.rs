use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// EVSE object with properties common to OCPP 2.0.1 and OCPP 2.1.0.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Identified Object. MRID. Numeric ID of the EVSE within the Charging Station.
    pub id: i32,

    /// An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

impl EVSEType {
    /// Creates a new `EVSEType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Numeric ID of the EVSE within the Charging Station
    ///
    /// # Returns
    ///
    /// A new instance of `EVSEType` with optional fields set to `None`
    pub fn new(id: i32) -> Self {
        Self {
            id,
            connector_id: None,
            custom_data: None,
        }
    }

    /// Sets the connector ID.
    ///
    /// # Arguments
    ///
    /// * `connector_id` - An id to designate a specific connector by connector index number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_connector_id(mut self, connector_id: i32) -> Self {
        self.connector_id = Some(connector_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this EVSE
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The numeric ID of the EVSE within the Charging Station
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Numeric ID of the EVSE within the Charging Station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the connector ID.
    ///
    /// # Returns
    ///
    /// An optional connector ID
    pub fn connector_id(&self) -> Option<i32> {
        self.connector_id
    }

    /// Sets the connector ID.
    ///
    /// # Arguments
    ///
    /// * `connector_id` - An id to designate a specific connector, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_connector_id(&mut self, connector_id: Option<i32>) -> &mut Self {
        self.connector_id = connector_id;
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
    /// * `custom_data` - Custom data for this EVSE, or None to clear
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
    fn test_new_evse() {
        let id = 42;

        let evse = EVSEType::new(id);

        assert_eq!(evse.id(), id);
        assert_eq!(evse.connector_id(), None);
        assert_eq!(evse.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = 42;
        let connector_id = 1;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let evse = EVSEType::new(id)
            .with_connector_id(connector_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(evse.id(), id);
        assert_eq!(evse.connector_id(), Some(connector_id));
        assert_eq!(evse.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = 42;
        let id2 = 43;
        let connector_id = 1;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut evse = EVSEType::new(id1);

        evse.set_id(id2)
            .set_connector_id(Some(connector_id))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(evse.id(), id2);
        assert_eq!(evse.connector_id(), Some(connector_id));
        assert_eq!(evse.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        evse.set_connector_id(None).set_custom_data(None);

        assert_eq!(evse.connector_id(), None);
        assert_eq!(evse.custom_data(), None);
    }
}
