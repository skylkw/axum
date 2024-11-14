use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;
        
        // 创建 IMAGE_LEVEL1 枚举类型
        tx.execute_unprepared(
            r#"CREATE TYPE IMAGE_LEVEL1 AS ENUM ('Optical', 'Infrared', 'SAR')"#,
        )
        .await?;
        
        // 创建 IMAGE_LEVEL2 枚举类型
        tx.execute_unprepared(
            r#"CREATE TYPE IMAGE_LEVEL2 AS ENUM ('Airplane', 'Vehicle')"#,
        )
        .await?;
        
        // 创建 image 表
        tx.execute_unprepared(
            r#"CREATE TABLE image (
                id BIGSERIAL NOT NULL PRIMARY KEY,
                filename TEXT NOT NULL,
                url TEXT NOT NULL,
                user_id UUID NOT NULL,
                level1 IMAGE_LEVEL1 NOT NULL,
                level2 IMAGE_LEVEL2 NOT NULL,
                create_at TIMESTAMPTZ DEFAULT current_timestamp,
                update_at TIMESTAMPTZ DEFAULT current_timestamp,
                CONSTRAINT fk_image_user FOREIGN KEY(user_id) REFERENCES users(id)
            )"#,
        )
        .await?;
        
        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;
        
        // 删除 image 表
        tx.execute_unprepared(r#"DROP TABLE IF EXISTS image"#).await?;
        
        // 删除 IMAGE_LEVEL2 枚举类型
        tx.execute_unprepared(r#"DROP TYPE IF EXISTS IMAGE_LEVEL2"#).await?;
        
        // 删除 IMAGE_LEVEL1 枚举类型
        tx.execute_unprepared(r#"DROP TYPE IF EXISTS IMAGE_LEVEL1"#).await?;
        
        tx.commit().await?;
        Ok(())
    }
}