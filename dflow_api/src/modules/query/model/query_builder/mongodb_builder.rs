use mongodb::bson::{doc, Document};

use crate::modules::dmodel::model::model::Model;

use super::{
    abstract_query::{AbstractQuery, QueryColumn},
    QueryBuilder,
};

pub struct MongoDbBuilder {
    pub model: Model,
}

/**
 * Rules and tips (for now):
 * - Embeded documents don't need lookup
 * - Always do and unwind for non nesting-level-0 fields (embeded docs, lookups, etc)
 * - Stages:
 * $lookup : {
 *      from: mainTable in join definition,
 *      localField: joinField in join definition,
 *      foreignField: mainField in join definition,
 *      as: mainTableName in join definition
 * },
 * {$unwind: [tableName]}
 * $group: {
 *      _id: {... all non-aggregated fields}
 *      [name]: {[aggregation]: [fieldName] | [embeded | lookup].fieldName | 1(for count)}
 * },
 */

impl MongoDbBuilder {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    /// TO-DO: provide a better mechanism to retrieve MainCollectionId (probably in the AbstractQuery model)
    pub fn get_main_collection_id<'a>(&self, query: &'a AbstractQuery) -> &'a usize {
        &query
            .columns
            .first()
            .expect("At least one column should be provided")
            .table_id
    }

    pub fn lookups(&self, query: &AbstractQuery, pipeline: &mut Vec<Document>) {
        let main_table_id = self.get_main_collection_id(query);
        for j in &query.joins {
            //Map joinDefinition to lookup:
            //Caution >> JOIN main_table (FOREIGN) ON A.main_field (FOREIGN_FIELD) = B.join_field (LOCAL_FIELD)
            //local
            let main_table = self.model.tables.get(&j.join_table_id).unwrap();
            let local_field = main_table.columns.get(&j.join_field_id).unwrap();
            //foreign
            let from = self.model.tables.get(&j.main_table_id).unwrap();
            let foreign_field = from.columns.get(&j.main_field_id).unwrap();

            //If localField is not the base document it must be referenced as [foreign_document].[field_name]
            let local_field_is_main_table = main_table_id != &j.join_table_id;
            let mut local_field_name = local_field.name.clone();
            if local_field_is_main_table {
                let value = format!("{}.{}", &main_table.name, local_field_name).to_owned();
                local_field_name = value;
            }
            let lookup = doc! {
                "$lookup": {
                    "from": &from.name,
                    "localField": &local_field_name,
                    "foreignField": &foreign_field.name,
                    "as": &from.name
                }
            };
            let unwind = doc! {
                "$unwind": format!("${}", &from.name)
            };
            pipeline.push(lookup);
            pipeline.push(unwind);
        }
    }

    pub fn group(&self, query: &AbstractQuery, pipeline: &mut Vec<Document>) {
        let main_table_id = self.get_main_collection_id(query);
        let group_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_none())
            .collect::<Vec<&QueryColumn>>();

        let mut id = Document::new();
        for g in group_fields {
            let mut field_name = g.column_name.clone();
            if main_table_id != &g.table_id {
                field_name = format!("{}.{}", &g.table_name, field_name).to_owned();
            }
            id.insert(&field_name.replace(".", "_"), format!("${}", &field_name));
        }

        let aggregated_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_some())
            .collect::<Vec<&QueryColumn>>();

        let mut group = Document::new();
        for a in aggregated_fields {
            let mut field_name = a.column_name.clone();
            if main_table_id != &a.table_id {
                field_name = format!("{}.{}", &a.table_name, field_name).to_owned();
            }
            let aggregation = doc! {
                "$sum": "$".to_owned() + &field_name
            };
            group.insert(&field_name.replace(".", "_"), aggregation);
        }
        group.insert("_id", id);
        pipeline.push(doc! {
            "$group": group
        });
    }

    pub fn project(&self, query: &AbstractQuery, pipeline: &mut Vec<Document>) {
        let main_table_id = self.get_main_collection_id(query);
        let group_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_none())
            .collect::<Vec<&QueryColumn>>();
        let mut project = Document::new();
        project.insert("_id", 0);
        let aggregated_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_some())
            .collect::<Vec<&QueryColumn>>();
        for g in group_fields {
            let mut field_name = g.column_name.clone();
            if main_table_id != &g.table_id {
                field_name = format!("{}.{}", &g.table_name, field_name).to_owned();
            }
            let mut field_value = field_name.clone(); 
            field_value = field_value.replace(".", "_");
            if aggregated_fields.len() > 0 {
                field_value = format!("_id.{}", field_value).to_owned();
            }
            project.insert(&field_name.replace(".", "_"), format!("${}", &field_value));
        }
        for a in aggregated_fields {
            let mut field_name = a.column_name.clone();
            if main_table_id != &a.table_id {
                field_name = format!("{}_{}", &a.table_name, field_name).to_owned();
            }
            project.insert(&field_name, 1);
        }
        pipeline.push(doc! {"$project": project})
    }
}

impl QueryBuilder for MongoDbBuilder {
    type Output = Vec<Document>;

    fn build(&self, query: &AbstractQuery) -> Self::Output {
        let mut pipeline: Vec<Document> = vec![];
        if !query.joins.is_empty() {
            self.lookups(query, &mut pipeline);
        }
        self.group(query, &mut pipeline);
        self.project(query, &mut pipeline);
        //let mut project = Document::new();

        pipeline
    }
}
