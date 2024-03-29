//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
  fn table_name(&self) -> &str {
    "auction"
  }
}

#[derive(
  Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize,
)]
pub struct Model {
  pub id: Uuid,
  pub created: Option<DateTimeWithTimeZone>,
  pub updated: Option<DateTimeWithTimeZone>,
  pub show_id: Uuid,
  pub product_id: Uuid,
  pub started_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
  Id,
  Created,
  Updated,
  ShowId,
  ProductId,
  StartedAt,
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
      Self::ShowId => ColumnType::Uuid.def(),
      Self::ProductId => ColumnType::Uuid.def(),
      Self::StartedAt => ColumnType::TimestampWithTimeZone.def().null(),
    }
  }
}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    match self {
      Self::Bid => Entity::has_many(super::bid::Entity).into(),
      Self::Product => Entity::belongs_to(super::product::Entity)
        .from(Column::ProductId)
        .to(super::product::Column::Id)
        .into(),
      Self::Show => Entity::belongs_to(super::show::Entity)
        .from(Column::ShowId)
        .to(super::show::Column::Id)
        .into(),
    }
  }
}

impl Related<super::bid::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Bid.def()
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
  #[sea_orm(entity = "super::product::Entity")]
  Product,
  #[sea_orm(entity = "super::show::Entity")]
  Show,
}
