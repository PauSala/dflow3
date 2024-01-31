use serde::{Deserialize, Serialize};
use crate::modules::dmodel::model::model::TypeAlias;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Aggregation {
    Sum,
    Avg,
    Max,
    Min,
    Count,
    CountDistinct,
}

impl Aggregation {
    pub fn get_value(&self) -> String {
        match self {
            Aggregation::Sum => "SUM".to_string(),
            Aggregation::Avg => "AVG".to_string(),
            Aggregation::Max => "MAX".to_string(),
            Aggregation::Min => "MIN".to_string(),
            Aggregation::Count => "COUNT".to_string(),
            Aggregation::CountDistinct => "COUNT".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Format {
    Year,
    Quarter,
    Month,
    Week,
    Day,
    DayHour,
    DayHourMinute,
    Timestamp,
    WeekDay,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Order {
    Asc,
    Desc,
}

impl Order {
    pub fn get_value(&self) -> String {
        match self {
            Order::Asc => String::from("ASC"),
            Order::Desc => String::from("DESC"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Operator {
    Eq,
    Ge,
    G,
    Se,
    Sm,
    NotEq,
    Like,
    In,
    Between,
}

impl Operator {
    pub fn get_value(&self) -> String {
        match self {
            Operator::Eq => String::from("="),
            Operator::Ge => String::from(">="),
            Operator::G => String::from(">"),
            Operator::Se => String::from("<="),
            Operator::Sm => String::from("<"),
            Operator::NotEq => String::from("<>"),
            Operator::Like => String::from("LIKE"),
            Operator::In => String::from("IN"),
            Operator::Between => String::from("BETWEEN"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum FilterValue {
    Number(f64),
    Text(String),
    Date(String),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum ValueContainer {
    UniValue(FilterValue),
    MultiValue(Vec<FilterValue>),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct QueryFilter {
    pub column_name: String,
    pub table_name: String,
    pub column_id: usize,
    pub table_id: usize,
    pub operator: Operator,
    pub value: ValueContainer,
    pub data_type: TypeAlias,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct QueryColumn {
    pub table_name: String,
    pub column_name: String,
    pub column_id: usize,
    pub table_id: usize,
    pub aggregation: Option<Aggregation>,
    pub format: Option<Format>,
    pub order: Option<Order>,
    pub data_type: TypeAlias,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct JoinDefinition {
    pub main_table_id: usize,
    pub join_table_id: usize,
    pub main_field_id: usize,
    pub join_field_id: usize 
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AbstractQuery<'a> {
    pub model_id: &'a str,
    pub columns: Vec<QueryColumn>,
    pub filters: Vec<QueryFilter>,
    pub joins: Vec<JoinDefinition>
}

impl AbstractQuery<'_> {
    pub fn get_columns(&self) -> Vec<QueryColumn> {
        self.columns.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AbstractQueryDto {
    pub columns: Vec<QueryColumn>,
    pub filters: Vec<QueryFilter>,
    pub joins: Vec<JoinDefinition>
}
