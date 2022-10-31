#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SetNetworkProfileStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Failed,
}
