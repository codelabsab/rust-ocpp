use serde::{Deserialize, Serialize};

/// This class does not get 'AdditionalProperties = false' in the schema generation,
/// so it can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomDataType {
    /// Vendor-specific identifier
    pub vendor_id: String,
}
