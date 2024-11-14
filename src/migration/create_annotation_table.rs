use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;

        // 创建 tags 表
        tx.execute_unprepared(
            r#"CREATE TABLE tags (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                level INT NOT NULL,
                parent_id BIGINT,
                CONSTRAINT fk_parent_tag FOREIGN KEY(parent_id) REFERENCES tags(id)
            )"#,
        )
        .await?;

        // 创建 annotation 表，引用 tags 表
        tx.execute_unprepared(
            r#"CREATE TABLE annotation (
                id UUID NOT NULL PRIMARY KEY,
                image_id BIGINT NOT NULL,
                user_id UUID NOT NULL,
                label TEXT NOT NULL,
                annotation_type BIGINT NOT NULL,
                tag_level1 BIGINT NOT NULL,
                tag_level2 BIGINT NOT NULL,
                tag_level3 BIGINT NOT NULL,
                tag_level4 BIGINT NOT NULL,
                tag_level5 BIGINT,
                content JSONB NOT NULL,
                create_at TIMESTAMPTZ DEFAULT current_timestamp,
                update_at TIMESTAMPTZ DEFAULT current_timestamp,
                CONSTRAINT fk_annotation_image FOREIGN KEY(image_id) REFERENCES image(id),
                CONSTRAINT fk_annotation_user FOREIGN KEY(user_id) REFERENCES users(id),
                CONSTRAINT fk_tag_level1 FOREIGN KEY(tag_level1) REFERENCES tags(id),
                CONSTRAINT fk_tag_level2 FOREIGN KEY(tag_level2) REFERENCES tags(id),
                CONSTRAINT fk_tag_level3 FOREIGN KEY(tag_level3) REFERENCES tags(id),
                CONSTRAINT fk_tag_level4 FOREIGN KEY(tag_level4) REFERENCES tags(id),
                CONSTRAINT fk_tag_level5 FOREIGN KEY(tag_level5) REFERENCES tags(id)
            )"#,
        )
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tx = db.begin().await?;

        // 删除 annotation 表
        tx.execute_unprepared(r#"DROP TABLE IF EXISTS annotation"#)
            .await?;

        // 删除 tags 表
        tx.execute_unprepared(r#"DROP TABLE IF EXISTS tags"#)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
}
