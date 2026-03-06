use anyhow::Context;
use sea_orm::DbConn;
use summer::error::Result;
use summer::{async_trait, tracing};

pub use inventory::submit;

/// Trait for custom migration registrars to collect migrations
pub trait MigrationRegistrar: Send + Sync + 'static {
    /// Register migrations with the given runner
    fn register(&self, runner: &mut MigrationRunner);
}

inventory::collect!(&'static dyn MigrationRegistrar);

/// Submit a migration registrar for automatic registration
#[macro_export]
macro_rules! submit_migration {
    ($ty:ident) => {
        ::summer_sea_orm_migration::migration::submit! {
            &$ty as &dyn ::summer_sea_orm_migration::MigrationRegistrar
        }
    };
}

/// Migration runner for executing all registered migrations
pub struct MigrationRunner {
    migrations: Vec<Box<dyn MigrationTrait>>,
}

impl MigrationRunner {
    /// Create a new migration runner
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Add a migration
    pub fn add_migration<M: MigrationTrait + 'static>(&mut self, migration: M) {
        self.migrations.push(Box::new(migration));
    }

    /// Run all registered migrations
    pub async fn run(&self, db: &DbConn) -> Result<()> {
        for migration in &self.migrations {
            tracing::info!(
                "Running migration: {}",
                migration.name()
            );
            migration.up(db).await.context("migration failed")?;
            tracing::info!(
                "Migration completed: {}",
                migration.name()
            );
        }
        Ok(())
    }

    /// Revert all migrations in reverse order
    pub async fn revert(&self, db: &DbConn) -> Result<()> {
        for migration in self.migrations.iter().rev() {
            tracing::info!(
                "Reverting migration: {}",
                migration.name()
            );
            migration.down(db).await.context("migration revert failed")?;
            tracing::info!(
                "Migration reverted: {}",
                migration.name()
            );
        }
        Ok(())
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait that migrations must implement
#[async_trait]
pub trait MigrationTrait: Send + Sync {
    /// Get the name of this migration
    fn name(&self) -> &str;

    /// Run the migration (up)
    async fn up(&self, db: &DbConn) -> Result<()>;

    /// Revert the migration (down)
    async fn down(&self, db: &DbConn) -> Result<()>;
}
