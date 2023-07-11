#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum APNAuthenticationEnumType {
    CHAP,
    NONE,
    PAP,
    #[default]
    AUTO,
}
