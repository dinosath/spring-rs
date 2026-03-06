use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

/// Auto-register a SeaORM migration struct.
///
/// The struct must implement the `MigrationTrait` trait with `up()` and `down()` methods.
/// Requires `summer-sea-orm-migration` plugin.
/// 
/// # Examples
/// ```rust,ignore
/// use summer_sea_orm_migration::MigrationTrait;
/// use summer_macros::migration;
/// use async_trait::async_trait;
///
/// #[migration]
/// pub struct CreateUsersTable;
///
/// #[async_trait]
/// impl MigrationTrait for CreateUsersTable {
///     fn name(&self) -> &str {
///         "m20240101_000001_create_users_table"
///     }
///
///     async fn up(&self, db: &sea_orm::DbConn) -> anyhow::Result<()> {
///         db.execute_unprepared("CREATE TABLE users (...)").await?;
///         Ok(())
///     }
///
///     async fn down(&self, db: &sea_orm::DbConn) -> anyhow::Result<()> {
///         db.execute_unprepared("DROP TABLE users").await?;
///         Ok(())
///     }
/// }
/// ```
pub fn migration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;
    let registrar_name = format_ident!("MigrationRegistrar{}", name);

    let expanded = quote! {
        #input

        #[allow(non_camel_case_types)]
        #vis struct #registrar_name;

        impl ::summer_sea_orm_migration::MigrationRegistrar for #registrar_name {
            fn register(&self, runner: &mut ::summer_sea_orm_migration::MigrationRunner) {
                runner.add_migration(#name);
            }
        }

        ::summer_sea_orm_migration::submit_migration!(#registrar_name);
    };

    TokenStream::from(expanded)
}
