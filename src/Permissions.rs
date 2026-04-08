use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::Types::PermissionMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    pub path: PathBuf,
    pub mode: PermissionMode,
}

pub struct PermissionsEngine {
    pub rules: Vec<PermissionRule>,
    pub base_dir: PathBuf,
}

impl PermissionsEngine {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            rules: Vec::new(),
            base_dir,
        }
    }

    pub fn add_rule(&mut self, path: PathBuf, mode: PermissionMode) {
        self.rules.push(PermissionRule { path, mode });
    }

    pub fn check_path(&self, target_path: &Path) -> PermissionMode {
        // Find the most specific matching rule
        let mut current_mode = PermissionMode::Manual;
        let mut max_depth = 0;

        for rule in &self.rules {
            if target_path.starts_with(&rule.path) {
                let depth = rule.path.components().count();
                if depth > max_depth {
                    max_depth = depth;
                    current_mode = rule.mode.clone();
                }
            }
        }
        current_mode
    }

    pub fn is_path_safe(&self, path: &Path) -> bool {
        // Ensure path is within allowed boundaries
        path.starts_with(&self.base_dir) && !path.to_string_lossy().contains("..")
    }
}

pub struct BashClassifier;

impl BashClassifier {
    pub fn classify(command: &str) -> bool {
        // Simplified version of the complex TS classifier
        let dangerous_keywords = ["rm ", "chmod", "chown", "sudo", "> /dev/"];
        dangerous_keywords.iter().any(|&k| command.contains(k))
    }
}
