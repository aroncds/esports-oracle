use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Match::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Match::GameId).string().not_null())
                    .col(ColumnDef::new(Match::Oracle).string().not_null())
                    .col(ColumnDef::new(Match::ExpireTime).big_unsigned().not_null())
                    .col(ColumnDef::new(Match::ExternalGameId).string().not_null())
                    .col(ColumnDef::new(Match::MasterPlayer).string())
                    .col(ColumnDef::new(Match::State).unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Match::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Match {
    Table,
    Id,
    GameId,
    Oracle,
    ExpireTime,
    ExternalGameId,
    MasterPlayer,
    State
}
