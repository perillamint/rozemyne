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
                    .table(BookFile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BookFile::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(BookFile::BookId).uuid().not_null())
                    .col(ColumnDef::new(BookFile::FilePath).string().not_null())
                    .col(
                        ColumnDef::new(BookFile::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BookFile::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_book_file_table_book_id")
                    .from(BookFile::Table, BookFile::BookId)
                    .to(Book::Table, Book::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_file_table_book_id")
                    .table(BookFile::Table)
                    .col(BookFile::BookId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_file_table_file_path")
                    .table(BookFile::Table)
                    .col(BookFile::FilePath)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_file_table_created_at")
                    .table(BookFile::Table)
                    .col(BookFile::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_file_table_updated_at")
                    .table(BookFile::Table)
                    .col(BookFile::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookFile::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum BookFile {
    Table,
    Id,
    BookId,
    FilePath,
    CreatedAt,
    UpdatedAt,
}
