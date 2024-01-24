use anyhow::{Ok, Result};

use crate::modules::dmodel::model::{
    model::{Model, TModelSaver},
    model_builder::TModelBuilder,
};

pub struct ModelSaverService;

impl ModelSaverService {
    pub(crate) fn new() -> Self {
        ModelSaverService {}
    }

    pub(crate) async fn run<Builder: TModelBuilder, Saver: TModelSaver>(
        &mut self,
        datasource_id: &str,
        model_id: &str,
        model_builder: &mut Builder,
        model_saver: &mut Saver,
    ) -> Result<Model> {
        let model = model_builder.build(datasource_id, model_id).await?;
        model_saver.persist(&model).await?;
        Ok(model)
    }
}
