pub use sea_orm_migration::prelude::*;

mod m20251126_025741_create_user_table;
mod m20251127_115730_add_auth_fields_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
                Box::new(m20251126_025741_create_user_table::Migration),
                Box::new(m20251127_115730_add_auth_fields_to_users::Migration),
        ]
    }
}
