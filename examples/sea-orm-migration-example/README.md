# SeaORM Migration Example

This example demonstrates how to use the SeaORM migration plugin with the `#[migration]` macro for automatic migration registration and manual execution.

## Features

- Automatic migration discovery using the `#[migration]` macro
- Manual migration control via `run_migrations()` function
- Simple registration pattern using the inventory crate
- Separate migration plugin for modularity
- Full async/await support

## Setup

### 1. Create a PostgreSQL database:

```bash
createdb sea_orm_migration_example
```

### 2. Update `config/app.toml` with your database credentials:

```toml
[sea-orm]
uri = "postgres://USER:PASSWORD@127.0.0.1:5432/sea_orm_migration_example"

[sea-orm-migration]
enable = true
migrate_at_start = true
```

### 3. Define migrations using the `#[migration]` macro:

```rust
use async_trait::async_trait;
use summer_macros::migration;
use summer_sea_orm_migration::MigrationTrait;

#[migration]
pub struct CreateUsersTable;

#[async_trait]
impl MigrationTrait for CreateUsersTable {
    fn name(&self) -> &str {
        "m20240101_000001_create_users_table"
    }

    async fn up(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared("CREATE TABLE users (...)").await?;
        Ok(())
    }

    async fn down(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared("DROP TABLE users").await?;
        Ok(())
    }
}
```

### 4. Run migrations manually in your code:

```rust
use summer_sea_orm_migration::run_migrations;

// In a route or startup hook
run_migrations(&db_connection).await?;
```

## Key Points

- Each migration struct is decorated with `#[migration]`
- Must implement `MigrationTrait` with `name()`, `up()`, and `down()` methods
- Migrations are automatically registered via the inventory crate
- Use `run_migrations(&db)` or `revert_migrations(&db)` for manual control
- Set `migrate_at_start = true` to auto-run on startup
- Separate `SeaOrmMigrationPlugin` keeps concerns isolated

## Plugins

This example uses:
- **SeaOrmPlugin**: Handles database connection
- **SeaOrmMigrationPlugin**: Migration infrastructure
- **WebPlugin**: HTTP server

## Running

```bash
cargo run -p sea-orm-migration-example
```

The application will:
1. Connect to the database via SeaOrmPlugin
2. Initialize migration infrastructure via SeaOrmMigrationPlugin
3. Auto-discover all `#[migration]` decorated structs
4. Allow you to call `run_migrations(&db)` to execute registered migrations
5. Start the web server on port 8000

Visit `http://localhost:8000/health` to verify the server is running.

## Manual vs Automatic Migration

The separation allows flexibility:
- **Manual**: Call `run_migrations(&db)` in routes or initialization code
- **Automatic**: Use a startup hook to run on app initialization
- **Selective**: Choose which migrations to run and when

## Cleanup

To revert all migrations:
```rust
use summer_sea_orm_migration::revert_migrations;

revert_migrations(&db_connection).await?;
```
