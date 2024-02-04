pub mod query_builder;
pub mod query_executor;

use self::{
    query_builder::{abstract_query::AbstractQuery, QueryBuilder},
    query_executor::{QueryResult, QueryRunner},
};
use anyhow::Result;

pub(crate) struct QueryHandler<B, E>
where
    B: QueryBuilder,
    E: QueryRunner<Input = B::Output>,
{
    builder: B,
    executor: E,
}

impl<B, E> QueryHandler<B, E>
where
    B: QueryBuilder,
    E: QueryRunner<Input = B::Output>,
{
    pub fn new(builder: B, executor: E) -> Self {
        QueryHandler { builder, executor }
    }
    pub async fn handle(&mut self, query: &AbstractQuery<'_>) -> Result<QueryResult> {
        let q = self.builder.build(query);
        let result = self.executor.run(q, query).await?;
        Ok(result)
    }
}
