use rocket::http::Status;
use rocket::{post, serde::json::Json};
use rocket_db_pools::Connection;

use serde::Deserialize;

use crate::template_dir::http500;
use crate::{
    modules::datasource::{
        application::DatasourceSaverService,
        infrastructure::persistence::datasource_saver::DataSourceSaver,
        model::datasource_type::{DataSource, DataSourceType},
    },
    template_dir::Error500Template,
    Db,
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) struct SaveDatasourceRequest<'a> {
    pub(crate) datasource_id: &'a str,
    pub(crate) datasource_name: &'a str,
    pub(crate) datasource_type: DataSourceType,
}

/// route: http://127.0.0.1:8000/datasource/save
/// Example request
/// ```
/// {
///     "datasource_id": "",
///     "datasource_name": "",
///     "datasource_type": {
///       "Sql": "Postgresql"
///     }
/// }
/// ```
#[post("/save", data = "<datasource_req>")]
pub(crate) async fn post_datasource_handler(
    mut db: Connection<Db>,
    datasource_req: Json<SaveDatasourceRequest<'_>>,
) -> Result<(), (Status, Error500Template)> {
    let datasource = DataSource::new(
        datasource_req.datasource_id.to_owned(),
        datasource_req.datasource_name.to_owned(),
        datasource_req.datasource_type.clone(),
    );

    let mut datastorer = DataSourceSaver::new(&mut db);
    let mut saver = DatasourceSaverService::new();
    let res = saver.run(datasource, &mut datastorer).await;
    res.map_err(|e| http500(e))
}
