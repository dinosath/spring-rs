use schemars::JsonSchema;
use serde::Deserialize;
use summer::config::Configurable;

summer::submit_config_schema!("sea-orm-migration", MigrationPluginConfig);

#[derive(Debug, Configurable, Clone, JsonSchema, Deserialize)]
#[config_prefix = "sea-orm-migration"]
pub struct MigrationPluginConfig {
    /// Enable or disable the migration plugin
    #[serde(default = "default_enable")]
    pub enable: bool,

    /// Run migrations automatically on startup
    #[serde(default = "default_migrate_at_start")]
    pub migrate_at_start: bool,
}

fn default_enable() -> bool {
    true
}

fn default_migrate_at_start() -> bool {
    true
}
