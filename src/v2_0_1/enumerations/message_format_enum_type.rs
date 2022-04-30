#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    UTF8,
}
