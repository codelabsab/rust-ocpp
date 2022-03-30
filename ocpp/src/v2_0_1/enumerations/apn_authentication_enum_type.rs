#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum APNAuthenticationEnumType {
    CHAP,
    NONE,
    PAP,
    AUTO,
}
