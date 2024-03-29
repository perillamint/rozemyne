use sea_orm_migration::prelude::*;

use crate::{m20230322_051542_create_book_table::Book, m20230322_061805_create_facet_table::Facet};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookXFacet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BookXFacet::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookXFacet::BookId).uuid().not_null())
                    .col(ColumnDef::new(BookXFacet::FacetId).uuid().not_null())
                    .col(
                        ColumnDef::new(BookXFacet::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BookXFacet::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_book_facet_book_id")
                    .from(BookXFacet::Table, BookXFacet::BookId)
                    .to(Book::Table, Book::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_book_facet_facet_id")
                    .from(BookXFacet::Table, BookXFacet::FacetId)
                    .to(Facet::Table, Facet::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_facet_book_id")
                    .table(BookXFacet::Table)
                    .col(BookXFacet::BookId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_facet_facet_id")
                    .table(BookXFacet::Table)
                    .col(BookXFacet::FacetId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_facet_created_at")
                    .table(BookXFacet::Table)
                    .col(BookXFacet::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_book_facet_updated_at")
                    .table(BookXFacet::Table)
                    .col(BookXFacet::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookXFacet::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum BookXFacet {
    Table,
    Id,
    BookId,
    FacetId,
    CreatedAt,
    UpdatedAt,
}
