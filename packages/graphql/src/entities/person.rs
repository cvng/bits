//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub created: DateTime,
  pub updated: Option<DateTime>,
  #[sea_orm(column_type = "Text")]
  pub email: String,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
  #[sea_orm(has_many = "super::bid::Entity")]
  Bid,
  #[sea_orm(has_many = "super::comment::Entity")]
  Comment,
  #[sea_orm(has_many = "super::show::Entity")]
  Show,
}

impl Related<super::bid::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Bid.def()
  }
}

impl Related<super::comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comment.def()
  }
}

impl Related<super::show::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Show.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, DeriveRelatedEntity, EnumIter)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::bid::Entity")]
  Bid,
  #[sea_orm(entity = "super::comment::Entity")]
  Comment,
  #[sea_orm(entity = "super::show::Entity")]
  Show,
}
