#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}
