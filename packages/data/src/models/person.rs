//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
  fn table_name(&self) -> &str {
    "person"
  }
}

#[derive(
  Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize,
)]
pub struct Model {
  pub id: Uuid,
  pub created: Option<DateTimeWithTimeZone>,
  pub updated: Option<DateTimeWithTimeZone>,
  pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
  Id,
  Created,
  Updated,
  Email,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
  Id,
}

impl PrimaryKeyTrait for PrimaryKey {
  type ValueType = Uuid;
  fn auto_increment() -> bool {
    false
  }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
  Bid,
  Comment,
  Product,
  Show,
}

impl ColumnTrait for Column {
  type EntityName = Entity;
  fn def(&self) -> ColumnDef {
    match self {
      Self::Id => ColumnType::Uuid.def(),
      Self::Created => ColumnType::TimestampWithTimeZone.def(),
      Self::Updated => ColumnType::TimestampWithTimeZone.def().null(),
      Self::Email => ColumnType::Text.def().unique(),
    }
  }
}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    match self {
      Self::Bid => Entity::has_many(super::bid::Entity).into(),
      Self::Comment => Entity::has_many(super::comment::Entity).into(),
      Self::Product => Entity::has_many(super::product::Entity).into(),
      Self::Show => Entity::has_many(super::show::Entity).into(),
    }
  }
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

impl Related<super::product::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Product.def()
  }
}

impl Related<super::show::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Show.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::bid::Entity")]
  Bid,
  #[sea_orm(entity = "super::comment::Entity")]
  Comment,
  #[sea_orm(entity = "super::product::Entity")]
  Product,
  #[sea_orm(entity = "super::show::Entity")]
  Show,
}