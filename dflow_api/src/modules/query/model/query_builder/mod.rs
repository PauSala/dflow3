pub mod abstract_query;
pub mod sql_builder;

use self::abstract_query::AbstractQuery;

///
/// `QueryBuilder<T>`
///
/// Trait to abstract queryBuilding.  
/// Recieves an `AbstractQuery`,   
/// Returns Self::Output
///
pub trait TQueryBuilder {
    type Output;
    fn build(&self, query: &AbstractQuery) -> Self::Output;
}
