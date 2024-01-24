pub mod sql_model_builder;
use anyhow::Result;

use self::sql_model_builder::SqlModelBuilder;
use super::model::Model;

pub(crate) enum ModelBuilder {
    Sql(SqlModelBuilder),
}

impl ModelBuilder {
    pub(crate) async fn build(&mut self, data_source_id: &str, model_id: &str) -> Result<Model> {
        match self {
            ModelBuilder::Sql(builder) => builder.build_model(data_source_id, model_id).await,
        }
    }
}
