use sea_orm_migration::prelude::*;

use crate::m20220902_200728_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FKEY_USER_IDP_TABLE_USER_ID: &str = "fkey_user_idp_table_user_id";

const IDX_USER_IDP_TABLE_USER_ID: &str = "idx_user_idp_table_user_id";
const IDX_USER_IDP_TABLE_IDP: &str = "idx_user_idp_table_idp";
const IDX_USER_IDP_TABLE_IDP_ID: &str = "idx_user_idp_table_idp_id";
const IDX_USER_IDP_TABLE_CREATED_AT: &str = "idx_user_idp_table_created_at";
const IDX_USER_IDP_TABLE_UPDATED_AT: &str = "idx_user_idp_table_updated_at";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserIdpId::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserIdpId::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserIdpId::UserId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserIdpId::Idp)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserIdpId::IdpId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserIdpId::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserIdpId::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Some Index or ForeginKey things.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FKEY_USER_IDP_TABLE_USER_ID)
                    .from(UserIdpId::Table, UserIdpId::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_IDP_TABLE_USER_ID)
                    .table(UserIdpId::Table)
                    .col(UserIdpId::UserId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_IDP_TABLE_IDP)
                    .table(UserIdpId::Table)
                    .col(UserIdpId::Idp)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_IDP_TABLE_IDP_ID)
                    .table(UserIdpId::Table)
                    .col(UserIdpId::IdpId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_IDP_TABLE_CREATED_AT)
                    .table(UserIdpId::Table)
                    .col(UserIdpId::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_IDP_TABLE_UPDATED_AT)
                    .table(UserIdpId::Table)
                    .col(UserIdpId::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserIdpId::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserIdpId {
    Table,
    Id,
    UserId,
    Idp,
    IdpId,
    CreatedAt,
    UpdatedAt,
}
