pub mod another_fake;
pub mod fake;
pub mod fake_classname_tags123;
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
        pub const EVENT_ACTOR: &str = "__actor__";
        pub const EVENT_RESOURCE_TYPE: &str = "__resource_type__";
        pub const EVENT_RESOURCE: &str = "__resource__";
        pub const EVENT_STATUS_CODE: &str = "__status_code__";
    }
}

#[async_trait::async_trait]
pub trait EventDispatcher {
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
