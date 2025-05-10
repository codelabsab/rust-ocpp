use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MessageFormatEnumType;

/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Message contents.
    #[validate(length(max = 512))]
    pub content: String,

    /// Required. Format of the message.
    pub format: MessageFormatEnumType,

    /// Required. Language identifier of the message content.
    #[validate(length(max = 8))]
    pub language: String,
}

impl MessageContentType {
    /// Creates a new `MessageContentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `content` - Message contents
    /// * `format` - Format of the message
    /// * `language` - Language identifier of the message content
    ///
    /// # Returns
    ///
    /// A new instance of `MessageContentType` with optional fields set to `None`
    pub fn new(content: String, format: MessageFormatEnumType, language: String) -> Self {
        Self {
            content,
            format,
            language,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this message content
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the message content.
    ///
    /// # Returns
    ///
    /// The message contents
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets the message content.
    ///
    /// # Arguments
    ///
    /// * `content` - Message contents
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    /// Gets the format of the message.
    ///
    /// # Returns
    ///
    /// The format of the message
    pub fn format(&self) -> &MessageFormatEnumType {
        &self.format
    }

    /// Sets the format of the message.
    ///
    /// # Arguments
    ///
    /// * `format` - Format of the message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_format(&mut self, format: MessageFormatEnumType) -> &mut Self {
        self.format = format;
        self
    }

    /// Gets the language identifier.
    ///
    /// # Returns
    ///
    /// The language identifier of the message content
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Sets the language identifier.
    ///
    /// # Arguments
    ///
    /// * `language` - Language identifier of the message content
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language(&mut self, language: String) -> &mut Self {
        self.language = language;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this message content, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_content() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();

        let message = MessageContentType::new(content.clone(), format.clone(), language.clone());

        assert_eq!(message.content(), content);
        assert_eq!(message.format(), &format);
        assert_eq!(message.language(), language);
        assert_eq!(message.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let message = MessageContentType::new(content.clone(), format.clone(), language.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(message.content(), content);
        assert_eq!(message.format(), &format);
        assert_eq!(message.language(), language);
        assert_eq!(message.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let content1 = "Please plug in your vehicle.".to_string();
        let content2 = "Charging session complete.".to_string();
        let format1 = MessageFormatEnumType::ASCII;
        let format2 = MessageFormatEnumType::HTML;
        let language1 = "en".to_string();
        let language2 = "fr".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut message =
            MessageContentType::new(content1.clone(), format1.clone(), language1.clone());

        message
            .set_content(content2.clone())
            .set_format(format2.clone())
            .set_language(language2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(message.content(), content2);
        assert_eq!(message.format(), &format2);
        assert_eq!(message.language(), language2);
        assert_eq!(message.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        message.set_custom_data(None);
        assert_eq!(message.custom_data(), None);
    }
}
