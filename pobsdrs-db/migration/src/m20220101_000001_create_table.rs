use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Name).string().not_null())
                    .col(ColumnDef::new(Game::Cover).string().not_null())
                    .col(ColumnDef::new(Game::Setup).string().not_null())
                    .col(ColumnDef::new(Game::Hints).string().not_null())
                    .col(ColumnDef::new(Game::Year).string().not_null())
                    .col(ColumnDef::new(Game::Version).string().not_null())
                    .col(ColumnDef::new(Game::Status).string().not_null())
                    .col(ColumnDef::new(Game::Added).string().not_null())
                    .col(ColumnDef::new(Game::Updated).string().not_null())
                    .to_owned(),
                )
                .await?;
        manager
            .create_table(
                Table::create()
                    .table(StoreLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StoreLink::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StoreLink::Url).string().not_null())
                    .col(ColumnDef::new(StoreLink::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DevGame::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DevGame::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DevGame::DevId).integer().not_null())
                    .col(ColumnDef::new(DevGame::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Dev::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dev::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dev::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RuntimeGame::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RuntimeGame::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RuntimeGame::RuntimeId).integer().not_null())
                    .col(ColumnDef::new(RuntimeGame::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Runtime::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Runtime::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Runtime::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(EngineGame::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EngineGame::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EngineGame::EngineId).integer().not_null())
                    .col(ColumnDef::new(EngineGame::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Engine::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Engine::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Engine::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
         manager
            .create_table(
                Table::create()
                    .table(TagGame::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TagGame::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TagGame::TagId).integer().not_null())
                    .col(ColumnDef::new(TagGame::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
          manager
            .create_table(
                Table::create()
                    .table(PubliGame::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PubliGame::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PubliGame::PubliId).integer().not_null())
                    .col(ColumnDef::new(PubliGame::GameId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Publi::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Publi::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Publi::Name).string().not_null())
                    .to_owned(),
            )
            .await
     }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StoreLink::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(DevGame::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(Dev::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(RuntimeGame::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(Runtime::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(EngineGame::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(Engine::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(TagGame::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(PubliGame::Table).to_owned())
            .await?;
         manager
            .drop_table(Table::drop().table(Publi::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Game {
    Table,
    Id,
    Name,
    Cover,
    Setup,
    Hints,
    Year,
    Version,
    Status,
    Added,
    Updated,
}

#[derive(Iden)]
enum StoreLink {
    Table,
    Id,
    Url,
    GameId,
}

#[derive(Iden)]
enum DevGame {
    Table,
    Id,
    DevId,
    GameId,
}

#[derive(Iden)]
enum Dev {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum RuntimeGame {
    Table,
    Id,
    RuntimeId,
    GameId,
}

#[derive(Iden)]
enum Runtime {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum EngineGame {
    Table,
    Id,
    EngineId,
    GameId,
}

#[derive(Iden)]
enum Engine {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum TagGame {
    Table,
    Id,
    TagId,
    GameId,
}

#[derive(Iden)]
enum Tag {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum PubliGame {
    Table,
    Id,
    PubliId,
    GameId,
}

#[derive(Iden)]
enum Publi {
    Table,
    Id,
    Name,
}
