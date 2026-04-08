/// Core Application Constants

pub const APP_NAME: &str = "claude-code-rs";
pub const APP_VERSION: &str = "0.1.0";
pub const DEFAULT_POLL_INTERVAL_MS: u64 = 1000;
pub const MAX_SESSION_HISTORY: usize = 1000;

pub const DEFAULT_SYSTEM_PROMPT: &str = 
    "You are a highly capable AI assistant operating locally. \
     You have access to advanced tools and permissions. \
     Prioritize safety and precision.";

pub const MAX_PAYLOAD_SIZE_BYTES: usize = 10 * 1024 * 1024; // 10MB
pub const DEFAULT_MODEL: &str = "claude-3-5-sonnet-20241022";
pub const FALLBACK_MODEL: &str = "claude-3-haiku-20240307";

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_OOM: i32 = 137;
