pub mod abstract_query;
pub mod sql_builder;

use self::{abstract_query::AbstractQuery, sql_builder::{postgres_builder::PostgresDialect, SqlQueryBuilder}};

///
/// `QueryBuilder<T>`
/// 
/// Trait to abstract queryBuilding.  
/// Recieves an `AbstractQuery`,   
/// Returns a string
/// 
pub trait QueryBuilder {
    fn build(&self, query: &AbstractQuery) -> String;
}

pub enum Builder{
    PgBuilder(SqlQueryBuilder<PostgresDialect>),
    /* MssqlBuilder(SqlQueryBuilder<MssqlDialect>) */
}

impl QueryBuilder for Builder {
    fn build(&self, query: &AbstractQuery) -> String {
        match self {
            Builder::PgBuilder(builder) => builder.build(query),
           /*  Builder::MssqlBuilder(builder) => builder.build(query), */
        }
    }
}
