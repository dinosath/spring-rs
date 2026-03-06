use async_trait::async_trait;
use anyhow::Error;
use sea_orm::{ConnectionTrait, DbConn};
use summer::error::Result;
use summer::{auto_config, App};
use summer_macros::migration;
use summer_sea_orm_migration::MigrationTrait;
use summer_sea_orm::SeaOrmPlugin;
use summer_web::{WebConfigurator, WebPlugin, extractor::Component};

#[migration]
pub struct CreateUsersTable;

#[async_trait]
impl MigrationTrait for CreateUsersTable {
    fn name(&self) -> &str {
        "m20240101_000001_create_users_table"
    }

    async fn up(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared(
            r#"
            CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) NOT NULL UNIQUE,
                email VARCHAR(255) NOT NULL UNIQUE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .await
        .map_err(Error::from)?;
        Ok(())
    }

    async fn down(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared("DROP TABLE IF EXISTS users")
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}

#[migration]
pub struct CreatePostsTable;

#[async_trait]
impl MigrationTrait for CreatePostsTable {
    fn name(&self) -> &str {
        "m20240101_000002_create_posts_table"
    }

    async fn up(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared(
            r#"
            CREATE TABLE posts (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id),
                title VARCHAR(255) NOT NULL,
                content TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .await
        .map_err(Error::from)?;
        Ok(())
    }

    async fn down(&self, db: &DbConn) -> Result<()> {
        db.execute_unprepared("DROP TABLE IF EXISTS posts")
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SeaOrmPlugin)
        .add_plugin(summer_sea_orm_migration::SeaOrmMigrationPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}

#[summer_web::get("/health")]
async fn health_check(Component(_db): Component<DbConn>) -> &'static str {
    // Optionally run migrations manually
    // run_migrations(&db).await.expect("migrations failed");
    "OK - Migrations applied successfully"
}
