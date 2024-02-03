use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use anyhow::Result;
use mongodb::bson::{doc, Document};
use mongodb::Client as MongoClient;
use rocket::futures::StreamExt;
use serde_json::{to_value, Value};

use crate::modules::dmodel::model::model::TypeAlias;
use crate::modules::{
    datasource::model::configurations::mongodb_configuration::MongoDbConfiguration,
    dmodel::model::model::Model,
};
pub struct MongoDbBuilder {
    client: MongoClient,
    config: MongoDbConfiguration,
}

impl MongoDbBuilder {
    pub(crate) fn new(config: MongoDbConfiguration, client: MongoClient) -> Self {
        Self { client, config }
    }
    pub(crate) async fn build_model(
        &mut self,
        datasource_id: &str,
        model_id: &str,
    ) -> Result<Model> {
        let mut model = Model::new(datasource_id, model_id);
        // Get a handle to a database.
        let db = self.client.database(&self.config.db_name);
        let collections: Vec<_> = db.list_collection_names(None).await?;
        for (i, c) in collections.iter().enumerate() {
            self.process_collection(c, &mut model, i).await?;
        }
        dbg!(&model);
        Ok(model)
    }

    async fn process_collection<'a>(
        &mut self,
        collection: &'a String,
        model: &'a mut Model,
        i: usize,
    ) -> Result<&'a mut Model> {
        let document = self
            .client
            .database(&self.config.db_name)
            .collection::<Document>(&collection);
        let stage_limit = doc! { "$limit": 10000 };
        let pipeline = vec![stage_limit];
        let mut cursor = document.aggregate(pipeline, None).await?;

        let mut collection_map: HashMap<String, HashSet<MgT>> = HashMap::new();

        while let Some(Ok(doc)) = cursor.next().await {
            let json: Value = to_value(&doc).unwrap();
            Self::infer_types_from_json(json, &mut collection_map, None, false);
        }

        model.add_table(&collection, i);
        dbg!(&collection_map);

        for (key, value) in collection_map {
            let mut sorted = value.iter().collect::<Vec<&MgT>>();
            sorted.sort();
            if let Some((type_alias, is_array)) = Self::map_from_mgt(sorted[0]) {
                model.add_column(
                    i,
                    &key,
                    type_alias.clone(),
                    &type_alias.to_string(),
                    is_array,
                )?;
            }
        }
        Ok(model)
    }

    fn map_from_mgt(mgt: &MgT) -> Option<(TypeAlias, bool)> {
        match mgt {
            MgT::String => Some((TypeAlias::Text, false)),
            MgT::Float => Some((TypeAlias::Float, false)),
            MgT::Integer => Some((TypeAlias::Integer, false)),
            MgT::Bool => Some((TypeAlias::Bool, false)),
            MgT::Date => Some((TypeAlias::Date, false)),
            MgT::Array(inner) => match inner.as_ref() {
                MgT::String => Some((TypeAlias::Text, true)),
                MgT::Float => Some((TypeAlias::Float, true)),
                MgT::Integer => Some((TypeAlias::Integer, true)),
                MgT::Bool => Some((TypeAlias::Bool, true)),
                MgT::Date => Some((TypeAlias::Date, true)),
                //Inner arrays not supported (yet?)
                MgT::Array(_) => None,
                MgT::Null => None,
            },
            MgT::Null => None,
        }
    }

    fn infer_types_from_json(
        json: Value,
        collection_map: &mut HashMap<String, HashSet<MgT>>,
        parent: Option<&str>,
        is_array: bool,
    ) {
        match json {
            Value::Object(obj) => {
                for (mut key, value) in obj {
                    if key == "$oid" || key == "$date" {
                        let entry = collection_map
                            .entry(parent.unwrap_or(&key).to_string())
                            .or_insert(HashSet::new());
                        let mongo_type;
                        if key == "$oid" {
                            mongo_type = MgT::String;
                        } else {
                            mongo_type = MgT::Date;
                        }
                        match is_array {
                            true => {
                                entry.insert(MgT::Array(Box::new(mongo_type)));
                            }
                            false => {
                                entry.insert(mongo_type);
                            }
                        }
                        continue;
                    }
                    if let Some(parent) = parent {
                        key = parent.to_owned() + "." + &key;
                    }
                    Self::infer_types_from_json(value, collection_map, Some(&key), is_array);
                }
            }
            Value::Array(values) => {
                for v in values {
                    Self::infer_types_from_json(v, collection_map, parent, true);
                }
            }
            _ => {
                if let Some(parent) = parent {
                    let mongo_type;
                    let entry = collection_map
                        .entry(parent.to_string())
                        .or_insert(HashSet::new());
                    match json {
                        Value::String(_) => {
                            mongo_type = MgT::String;
                        }
                        Value::Number(_) => {
                            if json.as_i64().is_some() || json.as_u64().is_some() {
                                mongo_type = MgT::Integer;
                            } else {
                                mongo_type = MgT::Float
                            }
                        }
                        Value::Bool(_) => {
                            mongo_type = MgT::Bool;
                        }
                        Value::Null => {
                            mongo_type = MgT::Null;
                        }
                        _ => panic!("This should never happen"),
                    }
                    match is_array {
                        true => {
                            entry.insert(MgT::Array(Box::new(mongo_type)));
                        }
                        false => {
                            entry.insert(mongo_type);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
enum MgT {
    String,
    Float,
    Integer,
    Bool,
    Date,
    Array(Box<MgT>),
    Null,
}
