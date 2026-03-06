//! [![summer-rs](https://img.shields.io/github/stars/summer-rs/summer-rs)](https://summer-rs.github.io/docs/plugins/summer-sea-orm-migration)
#![doc(html_favicon_url = "https://summer-rs.github.io/favicon.ico")]
#![doc(html_logo_url = "https://summer-rs.github.io/logo.svg")]

pub mod config;
pub mod migration;

pub use migration::{MigrationRegistrar, MigrationRunner, MigrationTrait};

use config::MigrationPluginConfig;
use sea_orm::DbConn;
use summer::config::ConfigRegistry;
use summer::plugin::ComponentRegistry;
use summer::{app::AppBuilder, error::Result, plugin::Plugin};
use summer::async_trait;

/// SeaORM Migration Plugin
/// 
/// Automatically discovers and runs migrations marked with #[migration] on startup.
/// 
/// # Example
/// 
/// ```rust,ignore
/// use summer::App;
/// use summer_sea_orm::SeaOrmPlugin;
/// use summer_sea_orm_migration::SeaOrmMigrationPlugin;
/// 
/// #[tokio::main]
/// async fn main() {
///     App::new()
///         .add_plugin(SeaOrmPlugin)
///         .add_plugin(SeaOrmMigrationPlugin)
///         .run()
///         .await
/// }
/// ```
pub struct SeaOrmMigrationPlugin;

#[async_trait]
impl Plugin for SeaOrmMigrationPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let config = app
            .get_config::<MigrationPluginConfig>()
            .expect("sea-orm migration plugin config load failed");

        if !config.enable {
            tracing::info!("SeaORM migration plugin disabled");
            return;
        }

        if config.migrate_at_start {
            let conn = app
                .get_component::<DbConn>()
                .expect("sea-orm db connection not exists");
            run_migrations(&conn)
                .await
                .expect("migrations failed");
        }
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<summer_sea_orm::SeaOrmPlugin>()]
    }
}

/// Run migrations using an injected database connection
/// 
/// This allows manual control over when migrations are executed.
pub async fn run_migrations(db: &sea_orm::DbConn) -> Result<()> {
    let mut runner = MigrationRunner::new();
    for registrar in inventory::iter::<&'static dyn MigrationRegistrar> {
        registrar.register(&mut runner);
    }
    runner.run(db).await
}

/// Revert all migrations
pub async fn revert_migrations(db: &sea_orm::DbConn) -> Result<()> {
    let mut runner = MigrationRunner::new();
    for registrar in inventory::iter::<&'static dyn MigrationRegistrar> {
        registrar.register(&mut runner);
    }
    runner.revert(db).await
}
