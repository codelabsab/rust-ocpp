pub enum RpcErrorCodes {
    FormatViolation,
    GenericError,
    InternalError,
    MessageTypeNotSupported,
    NotImplemented,
    NotSupported,
    OccurrenceConstraintViolation,
    PropertyConstraintViolation,
    ProtocolError,
    RpcFrameworkError,
    SecurityError,
    TypeConstraintViolation,
}

impl RpcErrorCodes {
    pub fn description(&self) -> &str {
        match self {
            RpcErrorCodes::FormatViolation => "Payload for Action is syntactically incorrect",
            RpcErrorCodes::GenericError => "Any other error not covered by the more specific error codes in this table",
            RpcErrorCodes::InternalError => "An internal error occurred and the receiver was not able to process the requested Action successfully" ,
            RpcErrorCodes::MessageTypeNotSupported => "A message with an Message Type Number received that is not supported by this implementation.",
            RpcErrorCodes::NotImplemented => "Requested Action is not known by receiver",
            RpcErrorCodes::NotSupported => "Requested Action is recognized but not supported by the receiver",
            RpcErrorCodes::OccurrenceConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints",
            RpcErrorCodes::PropertyConstraintViolation => "Payload is syntactically correct but at least one field contains an invalid value",
            RpcErrorCodes::ProtocolError => "Payload for Action is not conform the PDU structure",
            RpcErrorCodes::RpcFrameworkError => "Content of the call is not a valid RPC Request, for example: MessageId could not be read.",
            RpcErrorCodes::SecurityError => "During the processing of Action a security issue occurred preventing receiver from completing the Action successfully",
            RpcErrorCodes::TypeConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates data type constraints (e.g. \"somestring\": 12)",
        }
    }
}
