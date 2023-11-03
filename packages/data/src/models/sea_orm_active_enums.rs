//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

#![allow(clippy::enum_variant_names)]

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(
  Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "event_type")]
pub enum EventType {
  #[sea_orm(string_value = "auction_created")]
  AuctionCreated,
  #[sea_orm(string_value = "bid_created")]
  BidCreated,
  #[sea_orm(string_value = "comment_created")]
  CommentCreated,
  #[sea_orm(string_value = "person_created")]
  PersonCreated,
  #[sea_orm(string_value = "product_created")]
  ProductCreated,
  #[sea_orm(string_value = "show_created")]
  ShowCreated,
}
