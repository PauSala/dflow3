use mongodb::bson::{doc, Document};

use crate::modules::dmodel::model::model::Model;

use super::{
    abstract_query::{AbstractQuery, QueryColumn},
    QueryBuilder,
};

pub struct MongoDbQuery {
    pub main_document: String,
    pub pipeline: Vec<Document>,
}

impl MongoDbQuery {
    pub fn new(main_document: String, pipeline: Vec<Document>) -> Self {
        Self {
            main_document,
            pipeline,
        }
    }
}

pub struct MongoDbBuilder {
    pub model: Model,
}

/**
 * Rules and tips (for now):
 * - Embeded documents don't need lookup
 * - Always do and unwind for non nesting-level-0 fields (embeded docs, lookups, etc)
 * - Stages:
 * $lookup : {
 *      from: [mainTable in join definition],
 *      localField: [joinField in join definition],
 *      foreignField: [mainField in join definition],
 *      as: [mainTableName in join definition]
 * },
 * {$unwind: [tableName]}
 * $group: {
 *      _id: {... all non-aggregated fields}
 *      [name]: {[aggregation]: [fieldName] | [embeded | lookup].fieldName | 1(for count)}
 * },
 * - naming:
 *
 */

impl MongoDbBuilder {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    /// TO-DO: provide a better mechanism to retrieve MainCollectionId (probably modifying AbstractQuery definition)
    pub fn get_main_collection_id<'a>(&self, query: &'a AbstractQuery) -> &'a usize {
        &query
            .columns
            .first()
            .expect("At least one column should be provided")
            .table_id
    }

    pub fn lookups(
        &self,
        query: &AbstractQuery,
        main_collection_id: &usize,
        pipeline: &mut Vec<Document>,
    ) {
        for j in &query.joins {
            //Map joinDefinition to lookup:
            //This represents: LEFT JOIN main_table (FOREIGN) ON A.main_field (FOREIGN_FIELD) = B.join_field (LOCAL_FIELD)
            //local
            let main_table = self.model.tables.get(&j.join_table_id).unwrap();
            let local_field = main_table.columns.get(&j.join_field_id).unwrap();
            //foreign
            let from = self.model.tables.get(&j.main_table_id).unwrap();
            let foreign_field = from.columns.get(&j.main_field_id).unwrap();

            //If localField is not the base document it must be referenced as [foreign_document].[field_name]
            let local_field_is_main_table = main_collection_id != &j.join_table_id;
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

    pub fn get_field_name(&self, main_table_id: &usize, c: &QueryColumn) -> String {
        let mut field_name = c.column_name.clone();
        if main_table_id != &c.table_id {
            field_name = format!("{}.{}", &c.table_name, field_name).to_owned();
        }
        field_name
    }

    pub fn group(
        &self,
        main_collection_id: &usize,
        pipeline: &mut Vec<Document>,
        group_fields: &Vec<&QueryColumn>,
        aggregated_fields: &Vec<&QueryColumn>,
    ) {
        let mut id = Document::new();
        for g in group_fields {
            let field_name = self.get_field_name(main_collection_id, g);
            id.insert(&field_name.replace(".", "_"), format!("${}", &field_name));
        }

        let mut group = Document::new();
        for a in aggregated_fields {
            let field_name = self.get_field_name(main_collection_id, a);
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

    pub fn project(
        &self,
        main_collection_id: &usize,
        pipeline: &mut Vec<Document>,
        group_fields: &Vec<&QueryColumn>,
        aggregated_fields: &Vec<&QueryColumn>,
    ) {
        let mut project = Document::new();
        project.insert("_id", 0);

        for g in group_fields {
            let field_name = self.get_field_name(main_collection_id, g).replace(".", "_");
            let mut field_value = field_name.clone();
            if aggregated_fields.len() > 0 {
                field_value = format!("_id.{}", field_value).to_owned();
            }
            project.insert(&field_name, format!("${}", &field_value));
        }
        for a in aggregated_fields {
            let field_name = self.get_field_name(main_collection_id, a).replace(".", "_");
            project.insert(&field_name, 1);
        }
        pipeline.push(doc! {"$project": project})
    }
}

impl QueryBuilder for MongoDbBuilder {
    type Output = MongoDbQuery;

    fn build(&self, query: &AbstractQuery) -> Self::Output {
        let mut pipeline: Vec<Document> = vec![];
        let main_collection_id = self.get_main_collection_id(query);
        let group_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_none())
            .collect::<Vec<&QueryColumn>>();
        let aggregated_fields = query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_some())
            .collect::<Vec<&QueryColumn>>();

        if !query.joins.is_empty() {
            self.lookups(query, main_collection_id, &mut pipeline);
        };
        self.group(
            main_collection_id,
            &mut pipeline,
            &group_fields,
            &aggregated_fields,
        );
        self.project(
            main_collection_id,
            &mut pipeline,
            &group_fields,
            &aggregated_fields,
        );
        MongoDbQuery::new(
            query.columns.first().unwrap().table_name.to_owned(),
            pipeline,
        )
    }
}
