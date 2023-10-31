use seaography::heck::ToLowerCamelCase;
use seaography::EntityQueryFieldConfig;

/// Configuration object for the generated GraphQL schema.
pub struct BuilderContext;

impl BuilderContext {
  pub fn custom() -> seaography::BuilderContext {
    seaography::BuilderContext {
      entity_query_field: EntityQueryFieldConfig {
        type_name: Box::new(|object_name| {
          object_name.to_lower_camel_case() + "s"
        }),
        ..Default::default()
      },
      ..Default::default()
    }
  }
}
