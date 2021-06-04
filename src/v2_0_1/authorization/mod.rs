/*
    This Functional Block describes all the authorization-related functionalities, it contains different ways of authorizing a user, online
    and/or offline and the AuthorizeRequest message handling/behavior, Authorization Cache functionality, etc.
    When a user wishes to unplug the electric vehicle from the Charging Station, the Charging Station needs to verify that the user is
    either the one that initiated the charging or that the user is in the same group and thus allowed to terminate the charging. Once
    authorized, the Charging Station informs the CSMS that the charging has been stopped.
    • To improve the experience for users, a Charging Station MAY support local authorization of identifiers, using an
    Authorization Cache.
    • The LocalAuthorizeOffline Configuration Variable controls whether a Charging Station will authorize a user when
    offline using the Authorization Cache.
    • The LocalPreAuthorize Configuration Variable controls whether a Charging Station will use the Authorization Cache to
    start a transaction without performing an authorization with the CSMS.
*/

/// C 2.1 Authorization options
pub mod authorization_options;

/// C 2.2 ISO 15118 Authorization
pub mod iso_15118_authorization;

/// C 2.3. GroupId
pub mod group_id;

/// C 2.4 Cache
pub mod cache;

/// C 2.5 Local Authorization list
pub mod local_authorization_list;

/// C 2.6. Offline Authorization
pub mod offline_authorization;
