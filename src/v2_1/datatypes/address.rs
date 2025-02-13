use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// A generic address format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressType {
    /// Name of person/company
    pub name: String,

    /// Address line 1
    pub address1: String,

    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    /// City
    pub city: String,

    /// Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Country name
    pub country: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
