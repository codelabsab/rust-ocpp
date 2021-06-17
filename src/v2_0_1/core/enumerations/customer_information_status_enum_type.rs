#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Rejected,
    Invalid,
}
