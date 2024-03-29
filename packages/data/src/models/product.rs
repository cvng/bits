//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
  fn table_name(&self) -> &str {
    "product"
  }
}

#[derive(
  Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize,
)]
pub struct Model {
  pub id: Uuid,
  pub created: Option<DateTimeWithTimeZone>,
  pub updated: Option<DateTimeWithTimeZone>,
  pub creator_id: Uuid,
  pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
  Id,
  Created,
  Updated,
  CreatorId,
  Name,
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
  Auction,
  Person,
}

impl ColumnTrait for Column {
  type EntityName = Entity;
  fn def(&self) -> ColumnDef {
    match self {
      Self::Id => ColumnType::Uuid.def(),
      Self::Created => ColumnType::TimestampWithTimeZone.def(),
      Self::Updated => ColumnType::TimestampWithTimeZone.def().null(),
      Self::CreatorId => ColumnType::Uuid.def(),
      Self::Name => ColumnType::Text.def(),
    }
  }
}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    match self {
      Self::Auction => Entity::has_many(super::auction::Entity).into(),
      Self::Person => Entity::belongs_to(super::person::Entity)
        .from(Column::CreatorId)
        .to(super::person::Column::Id)
        .into(),
    }
  }
}

impl Related<super::auction::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Auction.def()
  }
}

impl Related<super::person::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Person.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::auction::Entity")]
  Auction,
  #[sea_orm(entity = "super::person::Entity")]
  Person,
}
