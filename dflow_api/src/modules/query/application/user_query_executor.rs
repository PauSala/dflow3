use crate::modules::query::model::{
    query_builder::{abstract_query::AbstractQuery, QueryBuilder},
    query_executor::{QueryExecutor, QueryResult},
};
use anyhow::Result;

pub(crate) async fn user_query_executor<T: QueryBuilder, U: QueryExecutor>(
    builder: T,
    mut executor: U,
    user_query: &AbstractQuery<'_>,
) -> Result<QueryResult> {
    let query = builder.build(user_query);
    let result = executor.run(&query, user_query).await?;
    Ok(result)
}
