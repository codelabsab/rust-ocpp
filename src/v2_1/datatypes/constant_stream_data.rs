use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, periodic_event_stream_params::PeriodicEventStreamParamsType,
};

/// Constant stream data type for periodic event streams.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct ConstantStreamDataType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Uniquely identifies the stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Parameters for the periodic event stream.
    pub params: PeriodicEventStreamParamsType,

    /// Id of monitor used to report this event. It can be a preconfigured or hardwired monitor.
    #[validate(range(min = 0))]
    pub variable_monitoring_id: i32,
}
