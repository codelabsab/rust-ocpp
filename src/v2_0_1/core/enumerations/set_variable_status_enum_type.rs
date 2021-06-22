#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
    RebootRequired,
}
