use anyhow::Result;
use crate::context::Context;

pub trait Migration {
    fn name(&self) -> &'static str;
    fn run(&self, ctx: &mut Context) -> Result<()>;
}

pub struct MigrationEngine {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationEngine {
    pub fn new() -> Self {
        Self {
            migrations: vec![
                Box::new(MigrateSonnet45To46),
                Box::new(MigrateOpusTo1m),
                Box::new(ResetAutoModeOptIn),
            ],
        }
    }

    pub fn run_all(&self, ctx: &mut Context) -> Result<()> {
        for migration in &self.migrations {
            migration.run(ctx)?;
        }
        Ok(())
    }
}

// Example Ported Migrations
struct MigrateSonnet45To46;
impl Migration for MigrateSonnet45To46 {
    fn name(&self) -> &'static str { "migrateSonnet45ToSonnet46" }
    fn run(&self, ctx: &mut Context) -> Result<()> {
        if ctx.model == "claude-3-5-sonnet-20240620" {
            ctx.model = "claude-3-5-sonnet-20241022".to_string();
        }
        Ok(())
    }
}

struct MigrateOpusTo1m;
impl Migration for MigrateOpusTo1m {
    fn name(&self) -> &'static str { "migrateOpusToOpus1m" }
    fn run(&self, ctx: &mut Context) -> Result<()> {
        if ctx.model == "claude-3-opus-20240229" {
            ctx.model = "claude-3-opus-latest".to_string();
        }
        Ok(())
    }
}

struct ResetAutoModeOptIn;
impl Migration for ResetAutoModeOptIn {
    fn name(&self) -> &'static str { "resetAutoModeOptIn" }
    fn run(&self, ctx: &mut Context) -> Result<()> {
        // Logic to reset specific flags in context
        Ok(())
    }
}
