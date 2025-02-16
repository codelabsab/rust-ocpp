use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

/// This class does not get 'AdditionalProperties = false' in the schema generation,
/// so it can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataType {
    /// Vendor-specific identifier
    #[validate(length(max = 255))]
    pub vendor_id: String,

    /// Additional vendor-specific properties
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub additional_properties: HashMap<String, Value>,
}

impl CustomDataType {
    pub fn new(vendor_id: String) -> Self {
        Self {
            vendor_id,
            additional_properties: HashMap::new(),
        }
    }
}
