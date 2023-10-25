//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "show")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub created: DateTimeWithTimeZone,
  pub updated: Option<DateTimeWithTimeZone>,
  pub creator_id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  pub started: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::auction::Entity")]
  Auction,
  #[sea_orm(has_many = "super::comment::Entity")]
  Comment,
  #[sea_orm(
    belongs_to = "super::person::Entity",
    from = "Column::CreatorId",
    to = "super::person::Column::Id",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Person,
}

impl Related<super::auction::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Auction.def()
  }
}

impl Related<super::comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comment.def()
  }
}

impl Related<super::person::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Person.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
