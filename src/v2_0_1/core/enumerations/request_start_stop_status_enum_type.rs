#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RequestStartStopStatusEnumType {
    Accepted,
    Rejected,
}
