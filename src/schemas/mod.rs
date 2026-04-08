use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Core Validation Schemas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSchema {
    pub name: String,
    pub rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String), // Regex pattern
}

pub struct SchemaValidator {
    schemas: std::collections::HashMap<String, ValidationSchema>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        Self { schemas: std::collections::HashMap::new() }
    }

    pub fn register_schema(&mut self, schema: ValidationSchema) {
        self.schemas.insert(schema.name.clone(), schema);
    }

    pub fn validate(&self, schema_name: &str, data: &str) -> Result<()> {
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| anyhow::anyhow!("Schema not found: {}", schema_name))?;

        for rule in &schema.rules {
            match rule {
                ValidationRule::Required => {
                    if data.trim().is_empty() {
                        return Err(anyhow::anyhow!("Validation failed: Field is required"));
                    }
                }
                ValidationRule::MinLength(min) => {
                    if data.len() < *min {
                        return Err(anyhow::anyhow!("Validation failed: Minimum length is {}", min));
                    }
                }
                ValidationRule::MaxLength(max) => {
                    if data.len() > *max {
                        return Err(anyhow::anyhow!("Validation failed: Maximum length is {}", max));
                    }
                }
                ValidationRule::Pattern(p) => {
                    let re = regex::Regex::new(p)?;
                    if !re.is_match(data) {
                        return Err(anyhow::anyhow!("Validation failed: Does not match pattern {}", p));
                    }
                }
            }
        }
        Ok(())
    }
}
