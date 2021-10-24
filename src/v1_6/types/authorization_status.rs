/// Status in a response to an AuthorizeRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum AuthorizationStatus {
    /// Identifier is allowed for charging.
    Accepted,
    /// Identifier has been blocked. Not allowed for charging.
    Blocked,
    /// Identifier has expired. Not allowed for charging.
    Expired,
    /// Identifier is unknown. Not allowed for charging.
    Invalid,
    /// Identifier is already involved in another transaction and multiple transactions are not allowed. (Only relevant for a StartTransaction.req.)
    ConcurrentTx,
}
