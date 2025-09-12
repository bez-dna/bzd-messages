pub use sea_orm_migration::prelude::*;

mod entities;

mod m20250912_141720_create_topics;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250912_141720_create_topics::Migration)]
    }
}
