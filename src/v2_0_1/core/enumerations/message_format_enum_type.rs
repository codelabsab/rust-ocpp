#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    UTF8,
}
