use crate::v2_1::datatypes::{CustomDataType, StreamDataElementType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// NotifyPeriodicEventStream message structure.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPeriodicEventStream {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub data: Vec<StreamDataElementType>,

    /// Id of stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Number of data elements still pending to be sent.
    #[validate(range(min = 0))]
    pub pending: i32,

    /// Base timestamp to add to time offset of values.
    pub basetime: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyPeriodicEventStream {
    /// Creates a new instance of the struct.
    ///
    /// * `data` - The data field
    /// * `id` - Id of stream.
    /// * `pending` - Number of data elements still pending to be sent.
    /// * `basetime` - Base timestamp to add to time offset of values.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(data: Vec<StreamDataElementType>, id: i32, pending: i32, basetime: DateTime<Utc>) -> Self {
        Self {
            data,
            id,
            pending,
            basetime,
            custom_data: None,
        }
    }

    /// Sets the data field.
    ///
    /// * `data` - The data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: Vec<StreamDataElementType>) -> &mut Self {
        self.data = data;
        self
    }

    /// Sets the id field.
    ///
    /// * `id` - Id of stream.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets the pending field.
    ///
    /// * `pending` - Number of data elements still pending to be sent.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_pending(&mut self, pending: i32) -> &mut Self {
        self.pending = pending;
        self
    }

    /// Sets the basetime field.
    ///
    /// * `basetime` - Base timestamp to add to time offset of values.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_basetime(&mut self, basetime: DateTime<Utc>) -> &mut Self {
        self.basetime = basetime;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// The data field
    pub fn get_data(&self) -> &Vec<StreamDataElementType> {
        &self.data
    }

    /// Gets a reference to the id field.
    ///
    /// # Returns
    ///
    /// Id of stream.
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    /// Gets a reference to the pending field.
    ///
    /// # Returns
    ///
    /// Number of data elements still pending to be sent.
    pub fn get_pending(&self) -> &i32 {
        &self.pending
    }

    /// Gets a reference to the basetime field.
    ///
    /// # Returns
    ///
    /// Base timestamp to add to time offset of values.
    pub fn get_basetime(&self) -> &DateTime<Utc> {
        &self.basetime
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
