use crate::modules::query::model::{
    query_builder::{abstract_query::AbstractQuery, TQueryBuilder},
    query_executor::{TQueryExecutor, QueryResult},
};
use anyhow::Result;

pub(crate) async fn user_query_executor<T: TQueryBuilder, U: TQueryExecutor>(
    builder: T,
    mut executor: U,
    user_query: &AbstractQuery<'_>,
) -> Result<QueryResult> {
    let query = builder.build(user_query);
    let result = executor.run(&query, user_query).await?;
    Ok(result)
}
