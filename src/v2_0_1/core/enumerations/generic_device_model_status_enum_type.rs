#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GenericDeviceModelStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}
