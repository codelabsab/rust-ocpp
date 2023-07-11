#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GenericDeviceModelStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}
