use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Advanced Skills Engine for Claude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub required_tools: Vec<String>,
    pub cost_weight: u32,
}

pub trait SkillExecutor: Send + Sync {
    fn definition(&self) -> &SkillDefinition;
    fn evaluate(&self, context: &str) -> Result<bool>;
    fn perform(&self, context: &mut String) -> Result<String>;
}

pub struct SkillsEngine {
    skills: Vec<Box<dyn SkillExecutor>>,
}

impl SkillsEngine {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn register(&mut self, skill: Box<dyn SkillExecutor>) {
        self.skills.push(skill);
    }

    pub fn suggest_skills(&self, input: &str) -> Vec<String> {
        self.skills.iter()
            .filter(|s| s.evaluate(input).unwrap_or(false))
            .map(|s| s.definition().name.clone())
            .collect()
    }

    pub fn execute_skill(&self, name: &str, context: &mut String) -> Result<String> {
        let skill = self.skills.iter().find(|s| s.definition().name == name)
            .ok_or_else(|| anyhow::anyhow!("Skill not found"))?;
        
        skill.perform(context)
    }
}
