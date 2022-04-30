#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GenericDeviceModelStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}
