use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    LocalBash,
    LocalAgent,
    RemoteAgent,
    InProcessTeammate,
    LocalWorkflow,
    MonitorMcp,
    Dream,
}

impl TaskType {
    pub fn prefix(&self) -> &'static str {
        match self {
            TaskType::LocalBash => "b",
            TaskType::LocalAgent => "a",
            TaskType::RemoteAgent => "r",
            TaskType::InProcessTeammate => "t",
            TaskType::LocalWorkflow => "w",
            TaskType::MonitorMcp => "m",
            TaskType::Dream => "d",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Killed,
}

impl TaskStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Killed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub id: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub description: String,
    pub tool_use_id: Option<String>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub total_paused_ms: u64,
    pub output_file: String,
    pub output_offset: u64,
    pub notified: bool,
}

pub fn generate_task_id(task_type: TaskType) -> String {
    let prefix = task_type.prefix();
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    format!("{}{}", prefix, suffix)
}

impl TaskState {
    pub fn new(task_type: TaskType, description: String) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        
        let id = generate_task_id(task_type.clone());
        
        Self {
            id,
            task_type,
            status: TaskStatus::Pending,
            description,
            tool_use_id: None,
            start_time,
            end_time: None,
            total_paused_ms: 0,
            output_file: String::new(), // In real app, this would be a path
            output_offset: 0,
            notified: false,
        }
    }
}
