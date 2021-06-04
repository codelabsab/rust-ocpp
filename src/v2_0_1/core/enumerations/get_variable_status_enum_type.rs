#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
}
