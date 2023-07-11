#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ConnectorStatusEnumType {
    #[default]
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}
