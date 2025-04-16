pub mod default;
pub mod info_repo;
pub mod repo;

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
