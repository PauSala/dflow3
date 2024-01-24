use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::{
    modules::datasource::{
        application::ConfigSaverService,
        infrastructure::persistence::sql_config_saver::SqlConfigurationSaver,
        model::configurations::sql_configuration::SqlConfig,
    },
    template_dir::{http500, Error500Template},
    Db,
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) enum ModelConfigReq {
    SqlConfigReq(SqlConfig),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) struct SaveConfigRequest {
    pub(crate) config_req: ModelConfigReq,
}

///
/// Save a configuration
/// route: http://127.0.0.1:8000/datasource/config/save
///```
/// {
///     "config_req": {
///         "SqlConfigReq": {
///           "datasource_id": "",
///           "host": "",
///           "port": 5432,
///           "user": "",
///           "password": "",
///           "db_name": "",
///           "schema": "public",
///           "dialect": "Postgresql"
///         }
///       }
/// }
///```
///
#[post("/config/save", data = "<config_req>")]
pub(crate) async fn post_config_handler(
    mut db: Connection<Db>,
    config_req: Json<SaveConfigRequest>,
) -> Result<(), (Status, Error500Template)> {

    let config_saver;
    match &config_req.config_req {
        ModelConfigReq::SqlConfigReq(sql_config) => {
            config_saver = SqlConfigurationSaver::new(&mut db, sql_config);
        }
    }

    let mut saver = ConfigSaverService::new();
    let res = saver.run(config_saver).await;
    res.map_err(|e| http500(e))
}
