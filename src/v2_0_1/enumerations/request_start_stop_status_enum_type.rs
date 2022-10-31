#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RequestStartStopStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
