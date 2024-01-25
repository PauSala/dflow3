pub mod abstract_query;
pub mod sql_builder;

use self::{
    abstract_query::AbstractQuery,
    sql_builder::{mssql_builder::MssqlDialect, postgres_builder::PostgresDialect, SqlQueryBuilder},
};

///
/// `QueryBuilder<T>`
///
/// Trait to abstract queryBuilding.  
/// Recieves an `AbstractQuery`,   
/// Returns a string
///
pub trait TQueryBuilder {
    fn build(&self, query: &AbstractQuery) -> String;
}

pub enum QueryBuilder {
    PgBuilder(SqlQueryBuilder<PostgresDialect>),
    MssqlBuilder(SqlQueryBuilder<MssqlDialect>)
}

impl TQueryBuilder for QueryBuilder {
    fn build(&self, query: &AbstractQuery) -> String {
        match self {
            QueryBuilder::PgBuilder(builder) => builder.build(query),
            QueryBuilder::MssqlBuilder(builder) => builder.build(query),
        }
    }
}
