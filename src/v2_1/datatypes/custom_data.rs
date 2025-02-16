use serde::{Deserialize, Serialize};
use validator::Validate;

/// This class does not get 'AdditionalProperties = false' in the schema generation,
/// so it can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataType {
    /// Vendor-specific identifier
    #[validate(length(max = 255))]
    pub vendor_id: String,
}
