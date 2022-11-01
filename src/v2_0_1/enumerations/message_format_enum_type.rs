#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    #[default]
    UTF8,
}
