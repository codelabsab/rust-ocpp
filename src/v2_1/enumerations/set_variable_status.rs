use serde::{Deserialize, Serialize};

/// Status of setting a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SetVariableStatusEnumType {
    /// Request has been accepted and the variable has been set.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Component specified in the request is unknown.
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,

    /// Variable specified in the request is unknown.
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,

    /// Attribute type specified in the request is not supported.
    #[serde(rename = "NotSupportedAttributeType")]
    NotSupportedAttributeType,

    /// Variable has been set but a reboot is required to apply the changes.
    #[serde(rename = "RebootRequired")]
    RebootRequired,
}
