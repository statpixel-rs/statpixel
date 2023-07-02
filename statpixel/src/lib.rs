use once_cell::sync::Lazy;
use std::collections::HashSet;

pub use translate::ApiError;
pub use translate::Error;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub static GUILDS: Lazy<tokio::sync::RwLock<HashSet<u64>>> = Lazy::new(Default::default);
pub static SHARDS: Lazy<tokio::sync::RwLock<u64>> = Lazy::new(Default::default);
