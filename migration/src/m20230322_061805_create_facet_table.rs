use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Facet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Facet::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Facet::ParentId).uuid())
                    .col(ColumnDef::new(Facet::Title).string().not_null())
                    .col(
                        ColumnDef::new(Facet::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Facet::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_facet_table_parent_id")
                    .table(Facet::Table)
                    .col(Facet::ParentId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_facet_table_title")
                    .table(Facet::Table)
                    .col(Facet::Title)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_facet_table_created_at")
                    .table(Facet::Table)
                    .col(Facet::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_facet_table_updated_at")
                    .table(Facet::Table)
                    .col(Facet::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Facet::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Facet {
    Table,
    Id,
    ParentId,
    Title,
    CreatedAt,
    UpdatedAt,
}
