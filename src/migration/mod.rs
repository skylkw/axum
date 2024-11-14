pub use sea_orm_migration::prelude::*;

mod create_annotation_table;
mod create_image_table;
mod create_message_table;
mod create_role_type;
mod create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_role_type::Migration),
            Box::new(create_user_table::Migration),
            Box::new(create_message_table::Migration),
            Box::new(create_image_table::Migration),
            Box::new(create_annotation_table::Migration),
        ]
    }
}
