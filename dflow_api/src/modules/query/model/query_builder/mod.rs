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

pub trait Builder {
    type Output;
    fn build(&self) -> Self::Output;
}

pub trait Executor {
    type Input;
    type Output; // New associated type for the output of execute method
    fn execute(&self, input: Self::Input);
}

struct StringBuilder;

impl Builder for StringBuilder {
    type Output = String;
    fn build(&self) -> Self::Output {
        String::from("HELLO!")
    }
}

struct StringExecutor;

impl Executor for StringExecutor {
    type Input = String;
    type Output = (); // Output type for StringExecutor's execute method

    fn execute(&self, input: Self::Input) {
        dbg!(input);
    }
}

fn build_and_execute<B, E>(builder: B, executor: E)
where
    B: Builder,
    E: Executor<Input = B::Output>, // Constrain E's Input type to match B's Output type
{
    let query = builder.build();
    executor.execute(query);
}

