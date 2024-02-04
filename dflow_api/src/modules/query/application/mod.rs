use anyhow::Result;
use super::model::{query_builder::QueryBuilder, query_executor::{QueryResult, QueryRunner}};
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;

pub(crate) struct QueryHandler<B, E>
where
    B: QueryBuilder,
    E: QueryRunner<Input = B::Output>,
{
    builder: B,
    runner: E,
}

impl<B, E> QueryHandler<B, E>
where
    B: QueryBuilder,
    E: QueryRunner<Input = B::Output>,
{
    pub fn new(builder: B, runner: E) -> Self {
        QueryHandler { builder, runner }
    }
    pub async fn handle(&mut self, query: &AbstractQuery<'_>) -> Result<QueryResult> {
        let q = self.builder.build(query);
        let result = self.runner.run(q, query).await?;
        Ok(result)
    }
}
