use std::collections::HashMap;

use mongodb::bson::{doc, Document};

use crate::modules::dmodel::model::model::Model;

use super::{
    abstract_query::{AbstractQuery, Aggregation, QueryColumn},
    QueryBuilder,
};

pub struct MongoDbQuery {
    pub main_document: String,
    pub columns: HashMap<String, QueryColumn>,
    pub pipeline: Vec<Document>,
}

impl MongoDbQuery {
    pub fn new(main_document: String, pipeline: Vec<Document>, columns: HashMap<String, QueryColumn>) -> Self {
        Self {
            main_document,
            pipeline,
            columns,
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
 * - $project {
 *  
 * }
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
            //=> LEFT JOIN main_table (FOREIGN) ON A.main_field (FOREIGN_FIELD) = B.join_field (LOCAL_FIELD)
            //local
            let main_table = self
                .model
                .tables
                .get(&j.join_table_id)
                .expect("This table should exist");
            let local_field = main_table
                .columns
                .get(&j.join_field_id)
                .expect("This column should exist");
            //foreign
            let from = self
                .model
                .tables
                .get(&j.main_table_id)
                .expect("This table should exist");
            let foreign_field = from
                .columns
                .get(&j.main_field_id)
                .expect("This column should exist");

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

    pub fn handle_aggregation(
        &self,
        agg: Option<Aggregation>,
        field_name: &str,
    ) -> Option<Document> {
        match agg {
            Some(agg) => match agg {
                Aggregation::Sum => Some(doc! {"$sum": "$".to_owned() + &field_name}),
                Aggregation::Avg => Some(doc! {"$avg": "$".to_owned() + &field_name}),
                Aggregation::Max => Some(doc! {"$max": "$".to_owned() + &field_name}),
                Aggregation::Min => Some(doc! {"$min": "$".to_owned() + &field_name}),
                Aggregation::Count => Some(doc! {"$sum": 1}),
                Aggregation::CountDistinct => None,
            },
            None => None,
        }
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
            self.handle_aggregation(a.aggregation, &field_name)
                .and_then(|agg| group.insert(&field_name.replace(".", "_"), agg));
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
    ) -> HashMap<String, QueryColumn> {
        let mut project = Document::new();
        //Don't send _id group by default
        project.insert("_id", 0);
        let mut col_map: HashMap<String, QueryColumn> = HashMap::new();

        for a in aggregated_fields {
            let field_name = self.get_field_name(main_collection_id, a).replace(".", "_");
            col_map.insert(field_name.clone(), (*a).clone());
            project.insert(&field_name, 1);
        }

        for g in group_fields {
            let field_name = self.get_field_name(main_collection_id, g).replace(".", "_");
            col_map.insert(field_name.clone(), (*g).clone());
            let field_value = format!("_id.{}", &field_name);
            project.insert(&field_name, format!("${}", &field_value));
        }
        pipeline.push(doc! {"$project": project});
        col_map
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
        let columns = self.project(
            main_collection_id,
            &mut pipeline,
            &group_fields,
            &aggregated_fields,
        );

        MongoDbQuery::new(
            query
                .columns
                .first()
                .expect("At least one element should exist")
                .table_name
                .to_owned(),
            pipeline,
            columns,
        )
    }
}
