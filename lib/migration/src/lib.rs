pub use sea_orm_migration::prelude::*;

mod entities;

mod m20250912_141720_create_topics;
mod m20250912_162515_create_messages;
mod m20250914_122002_create_streams;
mod m20250914_122217_create_messages_streams;
mod m20250914_122535_create_streams_users;
mod m20250914_122723_create_messages_topics;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250912_141720_create_topics::Migration),
            Box::new(m20250912_162515_create_messages::Migration),
            Box::new(m20250914_122002_create_streams::Migration),
            Box::new(m20250914_122217_create_messages_streams::Migration),
            Box::new(m20250914_122535_create_streams_users::Migration),
            Box::new(m20250914_122723_create_messages_topics::Migration),
        ]
    }
}
