use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(
                        ColumnDef::new(Book::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Book::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_table_title")
                    .table(Book::Table)
                    .col(Book::Title)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_table_created_at")
                    .table(Book::Table)
                    .col(Book::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_table_updated_at")
                    .table(Book::Table)
                    .col(Book::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Book {
    Table,
    Id,
    Title,
    CreatedAt,
    UpdatedAt,
}
