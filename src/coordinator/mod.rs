use anyhow::Result;
use crate::Task::TaskState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Central Coordination Engine for Swarm / Multiple Agents
pub struct Coordinator {
    pub active_tasks: Arc<Mutex<HashMap<String, TaskState>>>,
    pub max_concurrent_tasks: usize,
}

impl Coordinator {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_tasks: max_concurrent,
        }
    }

    pub fn dispatch_task(&self, mut task: TaskState) -> Result<String> {
        let mut lock = self.active_tasks.lock().unwrap();
        
        if lock.len() >= self.max_concurrent_tasks {
            return Err(anyhow::anyhow!("COORDINATOR ERROR: Max concurrent tasks reached"));
        }

        task.status = crate::Task::TaskStatus::Running;
        let id = task.id.clone();
        lock.insert(id.clone(), task);
        
        // In a real async runtime, this would spawn a tokio task
        Ok(id)
    }

    pub fn check_status(&self, id: &str) -> Option<crate::Task::TaskStatus> {
        let lock = self.active_tasks.lock().unwrap();
        lock.get(id).map(|t| t.status.clone())
    }

    pub fn cancel_task(&self, id: &str) -> Result<()> {
        let mut lock = self.active_tasks.lock().unwrap();
        if let Some(task) = lock.get_mut(id) {
            task.stop();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found"))
        }
    }
}
