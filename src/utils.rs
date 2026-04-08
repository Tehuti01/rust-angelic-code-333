use std::collections::VecDeque;
use uuid::Uuid;

// 1. CircularBuffer.ts -> CircularBuffer
pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn items(&self) -> &VecDeque<T> {
        &self.buffer
    }
}

// 2. Cursor.ts -> Cursor logic
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

// 3. abortController.ts -> Simple signal mock
pub struct AbortSignal {
    aborted: bool,
}

impl AbortSignal {
    pub fn new() -> Self {
        Self { aborted: false }
    }
    pub fn abort(&mut self) {
        self.aborted = true;
    }
    pub fn is_aborted(&self) -> bool {
        self.aborted
    }
}

// 4. array.ts -> Array helpers
pub fn unique<T: Eq + std::hash::Hash + Clone>(items: Vec<T>) -> Vec<T> {
    let set: std::collections::HashSet<_> = items.into_iter().collect();
    set.into_iter().collect()
}

// 5. agentId.ts -> ID generation
pub fn generate_agent_id() -> String {
    Uuid::new_v4().to_string()
}

// 6. activityManager.ts -> Activity tracking
pub struct ActivityManager {
    pub last_activity: std::time::Instant,
}

impl ActivityManager {
    pub fn new() -> Self {
        Self { last_activity: std::time::Instant::now() }
    }
    pub fn record(&mut self) {
        self.last_activity = std::time::Instant::now();
    }
}

// 7. env.ts -> Environment helpers
pub fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

// 8. platform.ts -> Platform detection
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") { "win32" }
    else if cfg!(target_os = "macos") { "darwin" }
    else { "linux" }
}

// 9. stringUtils.ts -> ANSI stripping
pub fn strip_ansi(text: &str) -> String {
    let re = regex::Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").unwrap();
    re.replace_all(text, "").to_string()
}

// 10. markdown.ts -> Simple markdown helpers
pub fn extract_links(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\[.*?\]\((https?://.*?)\)").unwrap();
    re.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}

// 11. retry.ts -> Simple retry mechanism
pub async fn retry<F, Fut, T, E>(mut f: F, max_attempts: usize) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        match f().await {
            Ok(val) => return Ok(val),
            Err(err) if attempts < max_attempts => {
                tokio::time::sleep(std::time::Duration::from_millis(100 * attempts as u64)).await;
            }
            Err(err) => return Err(err),
        }
    }
}

// 12. format.ts -> File size formatting
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB { format!("{:.2} GB", bytes as f64 / GB as f64) }
    else if bytes >= MB { format!("{:.2} MB", bytes as f64 / MB as f64) }
    else if bytes >= KB { format!("{:.2} KB", bytes as f64 / KB as f64) }
    else { format!("{} B", bytes) }
}

// 13. cwd.ts -> Directory helpers
pub fn resolve_path(path: &str) -> std::path::PathBuf {
    let p = std::path::Path::new(path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")).join(p)
    }
}

// 14. uuid.ts -> UUID generation
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// 15. hash.ts -> Simple hashing
pub fn hash_string(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut s = DefaultHasher::new();
    text.hash(&mut s);
    format!("{:x}", s.finish())
}
