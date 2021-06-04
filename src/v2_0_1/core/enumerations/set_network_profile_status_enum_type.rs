#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}
