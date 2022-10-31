#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RegistrationStatusEnumType {
    #[default]
    Accepted,
    Pending,
    Rejected,
}
