pub mod postgres_builder;
pub mod mssql_builder;

use crate::modules::dmodel::model::model::TypeAlias;

use super::{abstract_query::{AbstractQuery, FilterValue, QueryColumn, QueryFilter, ValueContainer}, TQueryBuilder};
use sql_query_builder::Select;


pub trait SqlBuilderDialect {
    fn select_date(&self, c: &QueryColumn) -> String;
    fn select_number(&self, c: &QueryColumn) -> String;
    fn select_text(&self, c: &QueryColumn) -> String;
    fn from_clause(&self, c: &QueryColumn, s: &Select) -> Select;
    fn join(&self,  select: Select, query: &AbstractQuery) -> Select;
    fn group_by(&self, select: Select, query: &AbstractQuery) -> Select;
    fn order_by(&self, select: Select, query: &AbstractQuery) -> Select;
    fn univalue_filter(&self, f: &QueryFilter, v: &FilterValue) -> String;
    fn multivalue_filter(&self, f: &QueryFilter, v: &Vec<FilterValue>) -> String;
    fn date_format(&self, c: &QueryColumn) -> String;
}

pub struct SqlQueryBuilder<T: SqlBuilderDialect> {
    pub dialect: T,
}

impl<T> SqlQueryBuilder<T>
where
    T: SqlBuilderDialect,
{
    fn join(&self, select: Select, query: &AbstractQuery) -> Select {
        self.dialect.join(select, &query)
    }

    fn select(&self, mut select: Select, query: &AbstractQuery) -> Select {
        for c in &query.columns {
            let content;
            match c.data_type {
                TypeAlias::Integer => content = self.dialect.select_number(&c),
                TypeAlias::Float => content = self.dialect.select_number(&c),
                TypeAlias::Text => content = self.dialect.select_text(&c),
                TypeAlias::Date => content = self.dialect.select_date(&c),
                TypeAlias::Bool => content = self.dialect.select_number(&c),
                TypeAlias::Array(_) => panic!("No arrays in sql for now")
            }
            select = select.select(&content)
        }
        select
    }

    fn where_clause(&self, mut select: Select, query: &AbstractQuery) -> Select {
        for filter in &query.filters {
            let clause;
            match &filter.value {
                ValueContainer::UniValue(v) => {
                    clause = self.dialect.univalue_filter(filter, v)
                }
                ValueContainer::MultiValue(v) => {
                    clause = self.dialect.multivalue_filter(filter, v)
                }
            }

            select = select.where_clause(&clause);
        }
        select
    }
}

impl<T> TQueryBuilder for SqlQueryBuilder<T>
where
    T: SqlBuilderDialect,
{
    fn build(&self, query: &AbstractQuery) -> String {
        //SELECT
        let mut select = Select::new();
        select = self.select(select, &query);
        //FROM
        let origin = &query.columns[0];
        select = self.dialect.from_clause(origin, &mut select);
        //JOIN
        select = self.join(select, &query);
        //WHERE
        select = self.where_clause(select, &query);
        //GROUP_BY
        if query.columns.iter().any(|c| c.aggregation.is_some()) {
            select = self.dialect.group_by(select, &query);
        }
        //ORDER_BY
        select = self.dialect.order_by(select, &query);
        select.debug().as_string()
    }
}
