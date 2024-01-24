use anyhow::{Ok, Result};

use crate::
    modules::
        dmodel::model::{
            model::{Model, ModelSaver},
            model_builder::ModelBuilder,
        }
    
;

pub struct ModelSaverService;

impl ModelSaverService {
    pub(crate) fn new() -> Self {
        ModelSaverService {}
    }

    pub(crate) async fn run<Saver: ModelSaver>(
        &mut self,
        datasource_id: &str,
        model_id: &str,
        model_builder: &mut ModelBuilder,
        model_saver: &mut Saver,
    ) -> Result<Model> {
        let model = model_builder.build(datasource_id, model_id).await?;
        model_saver.persist(&model).await?;
        Ok(model)
    }
}
