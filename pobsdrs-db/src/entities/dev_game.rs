//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "dev_game")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dev_id: i32,
    pub game_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dev::Entity",
        from = "Column::DevId",
        to = "super::dev::Column::Id"
    )]
    Dev,
    #[sea_orm(
        belongs_to = "super::game::Entity",
        from = "Column::GameId",
        to = "super::game::Column::Id"
    )]
    Game,
}

impl ActiveModelBehavior for ActiveModel {}