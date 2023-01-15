use heck::AsTitleCase;

/// Version pulled from Cargo.toml at compile time
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const NAME: &str = env!("CARGO_PKG_NAME");

/// default user-agent
pub(super) fn user_agent() -> String {
    format!("{}/{}", AsTitleCase(NAME), VERSION)
}

/// default threads value
pub(super) fn threads() -> i8 {
    1
}

/// default timeout value
pub(super) fn timeout() -> u64 {
    40
}
