pub mod pet;
pub mod store;
pub mod user;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Authorization {
    Authorized,
    Forbidden,
}

pub mod event {
    /// Anything to be recorded.
    pub type Event = std::collections::HashMap<String, String>;

    pub mod convention {
        pub const EVENT_SERVICE: &str = "_service_";
        pub const EVENT_ACTOR: &str = "_actor_";
        pub const EVENT_ACTION: &str = "_action_";
        pub const EVENT_RESOURCE_TYPE: &str = "_resource_type_";
        pub const EVENT_RESOURCE: &str = "_resource_";
        pub const EVENT_STATUS_CODE: &str = "_status_code_";
        pub const EVENT_LATENCY: &str = "_latency_";
        pub const EVENT_TIMESTAMP: &str = "timestamp";
    }
}

#[async_trait::async_trait]
pub trait EventDispatcher {
    fn service_name(&self) -> String;
    async fn dispatch(&self, event: event::Event);
}

/// API Key Authentication - Header.
#[async_trait::async_trait]
pub trait ApiKeyAuthHeader {
    type Claims;

    /// Extracting Claims from Header. Return None if the Claims is invalid.
    async fn extract_claims_from_header(
        &self,
        headers: &axum::http::header::HeaderMap,
        key: &str,
    ) -> Option<Self::Claims>;
}
