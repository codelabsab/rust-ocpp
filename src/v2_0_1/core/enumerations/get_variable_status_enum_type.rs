#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
}
