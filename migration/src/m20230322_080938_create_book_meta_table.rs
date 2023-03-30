use sea_orm_migration::prelude::*;

use crate::m20230322_051542_create_book_table::Book;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookMeta::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BookMeta::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(BookMeta::BookId).uuid().not_null())
                    .col(ColumnDef::new(BookMeta::Key).string().not_null())
                    .col(ColumnDef::new(BookMeta::Value).string().not_null())
                    .col(ColumnDef::new(BookMeta::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(BookMeta::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_book_meta_book_id")
                    .from(BookMeta::Table, BookMeta::BookId)
                    .to(Book::Table, Book::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_meta_book_id")
                    .table(BookMeta::Table)
                    .col(BookMeta::BookId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_meta_key")
                    .table(BookMeta::Table)
                    .col(BookMeta::Key)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_meta_created_at")
                    .table(BookMeta::Table)
                    .col(BookMeta::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_meta_updated_at")
                    .table(BookMeta::Table)
                    .col(BookMeta::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookMeta::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum BookMeta {
    Table,
    Id,
    BookId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
}
