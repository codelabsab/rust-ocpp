#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ConnectorStatusEnumType {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}
