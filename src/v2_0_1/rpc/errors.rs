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
    fn description(&self) -> &str {
        match self {
            FormatViolation => "Payload for Action is syntactically incorrect",
            GenericError => "Any other error not covered by the more specific error codes in this table",
            InternalError => "An internal error occurred and the receiver was not able to process the requested Action successfully" ,
            MessageTypeNotSupported => "A message with an Message Type Number received that is not supported by this implementation.",
            NotImplemented => "Requested Action is not known by receiver",
            NotSupported=> "Requested Action is recognized but not supported by the receiver",
            OccurrenceConstraintViolation=> "Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints",
            PropertyConstraintViolation=> "Payload is syntactically correct but at least one field contains an invalid value",
            ProtocolError=> "Payload for Action is not conform the PDU structure",
            RpcFrameworkError=> "Content of the call is not a valid RPC Request, for example: MessageId could not be read.",
            SecurityError=> "During the processing of Action a security issue occurred preventing receiver from completing the Action successfully",
            TypeConstraintViolation=> "Payload for Action is syntactically correct but at least one of the fields violates data type constraints (e.g. \"somestring\": 12)",
        }
    }
}

mod test {
    use crate::v2_0_1::rpc::errors::RpcErrorCodes;

    #[test]
    fn test_rpc() {
        let error = RpcErrorCodes::FormatViolation;
        let format_violation_desc = error.description();
        println!("{}", format_violation_desc);
    }
}
