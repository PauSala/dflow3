use rocket::{http::Status, post, response::status::BadRequest, serde::json::Json};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::{
    modules::datasource::{
        application::ConfigSaverService,
        infrastructure::persistence::sql_config_saver::SqlConfigurationSaver,
        model::configurations::sql_configuration::SqlConfig,
    }, template_dir::{http500, Error500Template}, Db
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) enum ModelConfigReq {
    SqlConfigReq(SqlConfig),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) struct SaveConfigRequest {
    pub(crate) config_req: ModelConfigReq,
}
#[post("/config/save", data = "<config_req>")]
pub(crate) async fn post_config_handler(
    mut db: Connection<Db>,
    config_req: Json<SaveConfigRequest>,
) -> Result<(),  (Status, Error500Template)> {
    let config_storer;
    match &config_req.config_req {
        ModelConfigReq::SqlConfigReq(sql_config) => {
            config_storer = SqlConfigurationSaver::new(&mut db, sql_config.clone());
        }
    }

    let mut saver = ConfigSaverService::new();
    let res = saver.run(config_storer).await;
    res.map_err(|e| http500(e))
}
