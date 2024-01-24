use sql_query_builder::Select;

use crate::modules::{
    dmodel::model::model::{Model, TypeAlias},
    query::model::query_builder::abstract_query::{
        AbstractQuery, Aggregation, FilterValue, Format, Operator, QueryColumn, QueryFilter,
    },
};

use super::SqlBuilderDialect;

pub struct PostgresDialect {
    pub model: Model,
    pub schema: String,
}

impl PostgresDialect {}

impl SqlBuilderDialect for PostgresDialect {
    //TO-DO -> Column ids should be table_id + column_id to avoid colisions
    fn select_date(&self, c: &QueryColumn) -> String {
        let format;
        match c.format {
            Some(Format::Year) => format = String::from("'YYYY'"),
            Some(Format::Quarter) => format = String::from("'YYYY-\"Q\"Q'"),
            Some(Format::Month) => format = String::from("'YYYY-MM'"),
            Some(Format::Week) => format = String::from("'IYYY-IW'"),
            Some(Format::Day) => format = String::from("'YYYY-MM-DD'"),
            Some(Format::DayHour) => format = String::from("'YYYY-MM-DD HH'"),
            Some(Format::DayHourMinute) => format = String::from("'YYYY-MM-DD HH:MI'"),
            Some(Format::Timestamp) => format = String::from("'YYYYY-MM-DD HH:MI:SS'"),
            Some(Format::WeekDay) => format = String::from("'ID'"),
            None => format = String::from("'YYYY-MM-DD'"),
        }
        format!(
            "to_char(\"{}\".\"{}\".\"{}\", {}) as \"{}\"",
            self.schema, c.table_name, c.column_name, format, c.column_name
        )
        .to_string()
    }

    fn select_number(&self, c: &QueryColumn) -> String {
        let content;
        match c.aggregation {
            Some(agg) => match agg {
                Aggregation::CountDistinct => match c.data_type {
                    TypeAlias::Integer | TypeAlias::Float => {
                        content = format!(
                        "ROUND ({}(DISTINCT \"{}\".\"{}\".\"{}\")::numeric, 4)::float as \"{}\"",
                        agg.get_value(),
                        self.schema,
                        c.table_name,
                        c.column_name,
                        c.column_name
                    )
                    }
                    _ => {
                        content = format!(
                            "{}(DISTINCT \"{}\".\"{}\".\"{}\") as \"{}\"",
                            agg.get_value(),
                            self.schema,
                            c.table_name,
                            c.column_name,
                            c.column_name
                        )
                    }
                },
                _ => {
                    content = format!(
                        "ROUND ({}(\"{}\".\"{}\".\"{}\")::numeric, 4)::float as \"{}\"",
                        agg.get_value(),
                        self.schema,
                        c.table_name,
                        c.column_name,
                        c.column_name
                    )
                }
            },
            None => {
                content = format!(
                    "ROUND ((\"{}\".\"{}\".\"{}\")::numeric, 4)::float as \"{}\"",
                    self.schema, c.table_name, c.column_name, c.column_name
                )
            }
        }
        content
    }

    fn select_text(&self, c: &QueryColumn) -> String {
        format!(
            "\"{}\".\"{}\".\"{}\" as \"{}\"",
            self.schema, c.table_name, c.column_name, c.column_name
        )
    }

    fn univalue_filter(&self, f: &QueryFilter, v: &FilterValue) -> String {
        let clause;
        match f.operator {
            Operator::Between | Operator::In => {
                panic!("Between and In operators can not be univalue")
            }
            _ => match v {
                FilterValue::Number(v) => {
                    clause = format!(
                        "\"{}\".\"{}\".\"{}\" {} {v}",
                        self.schema,
                        f.table_name,
                        f.column_name,
                        f.operator.get_value()
                    )
                }
                FilterValue::Text(v) => {
                    let v = v.replace("'", "''");
                    clause = format!(
                        "\"{}\".\"{}\".\"{}\" {} '{v}'",
                        self.schema,
                        f.table_name,
                        f.column_name,
                        f.operator.get_value()
                    )
                }
                FilterValue::Date(v) => {
                    clause = format!(
                        "\"{}\".\"{}\".\"{}\" {} '{v}'",
                        self.schema,
                        f.table_name,
                        f.column_name,
                        f.operator.get_value()
                    )
                }
            },
        }
        clause
    }

    fn multivalue_filter(&self, f: &QueryFilter, v: &Vec<FilterValue>) -> String {
        let clause;
        match v[0] {
            FilterValue::Number(_) => {
                let values = v
                    .iter()
                    .map(|v| match v {
                        FilterValue::Number(v) => format!("{}", v),
                        _ => {
                            panic!("This vector should contain only Int or Float")
                        }
                    })
                    .collect::<Vec<_>>();
                match f.operator {
                    Operator::In => {
                        clause = format!(
                            "\"{}\".\"{}\".\"{}\" {} ({})",
                            self.schema,
                            f.table_name,
                            f.column_name,
                            f.operator.get_value(),
                            values.join(",")
                        )
                    }
                    Operator::Between => {
                        clause = format!(
                            "\"{}\".\"{}\".\"{}\" {} {}",
                            self.schema,
                            f.table_name,
                            f.column_name,
                            f.operator.get_value(),
                            values.join(" AND ")
                        )
                    }
                    _ => panic!("Only Between and In are allowed here"),
                }
            }
            FilterValue::Text(_) | FilterValue::Date(_) => {
                let values = v
                    .iter()
                    .map(|v| match v {
                        FilterValue::Date(v) => format!("{}", v),
                        FilterValue::Text(v) => format!("{}", v),
                        _ => {
                            panic!("This vector should contain only Text or Date")
                        }
                    })
                    .collect::<Vec<_>>();

                match f.operator {
                    Operator::In => {
                        clause = format!(
                            "\"{}\".\"{}\".\"{}\" {} ({})",
                            self.schema,
                            f.table_name,
                            f.column_name,
                            f.operator.get_value(),
                            values
                                .iter()
                                .map(|v| {
                                    let n = v.replace("'", "''");
                                    format!("'{}'", n)
                                })
                                .collect::<Vec<String>>()
                                .join(",")
                        )
                    }
                    Operator::Between => {
                        clause = format!(
                            "\"{}\".\"{}\".\"{}\" {} ({})",
                            self.schema,
                            f.table_name,
                            f.column_name,
                            f.operator.get_value(),
                            values
                                .iter()
                                .map(|v| {
                                    let n = v.replace("'", "''");
                                    format!("'{}'", n)
                                })
                                .collect::<Vec<String>>()
                                .join("AND")
                        )
                    }
                    _ => panic!("Only Between and In are allowed here"),
                }
            }
        }
        clause
    }

    fn from_clause(&self, c: &QueryColumn, s: &Select) -> Select {
        s.clone().from(&format!("\"{}\"", c.table_name))
    }

    fn join(&self, mut select: Select, query: &AbstractQuery) -> Select {
        let joins = &query.joins;
        for join in joins {
            let pk_table = self
                .model
                .tables
                .get(&join.main_table_id)
                .expect("table should exist");
            let fk_table = self
                .model
                .tables
                .get(&join.join_table)
                .expect("table should exist");
            let pk_column = &pk_table
                .columns
                .get(&join.main_field)
                .expect("Column should exist")
                .name;
            let fk_column = &fk_table
                .columns
                .get(&join.join_field)
                .expect("Column should exist")
                .name;
            let pk_table = &pk_table.name;
            let fk_table = &fk_table.name;
            let join = format!(
                "\"{schema}\".\"{pk_table}\" ON \"{schema}\".\"{pk_table}\".\"{pk_column}\" = \"{schema}\".\"{fk_table}\".\"{fk_column}\"",
                schema = self.schema,
            );
            select = select.inner_join(&join);
        }
        select
    }

    fn group_by(&self, mut select: Select, query: &AbstractQuery) -> Select {
        for col in query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_none())
            .collect::<Vec<_>>()
            .iter()
        {
            select = select.group_by(&format!(
                "\"{}\".\"{}\".\"{}\"",
                self.schema, col.table_name, col.column_name
            ));
        }
        select
    }

    fn order_by(&self, mut select: Select, query: &AbstractQuery) -> Select {
        for col in query
            .columns
            .iter()
            .filter(|c| c.order.is_some())
            .collect::<Vec<_>>()
            .iter()
        {
            select = select.order_by(&format!(
                "\"{}\".\"{}\".\"{}\" {}",
                self.schema,
                col.table_name,
                col.column_name,
                &col.order.as_ref().expect("Should be filtered").get_value()
            ));
        }
        select
    }
}

#[cfg(test)]
pub mod tests {

    use crate::modules::{
        dmodel::model::model::TypeAlias, query::model::query_builder::abstract_query::Order,
    };

    use super::*;

    #[test]
    fn select_should_work_for_numeric_data() {
        let model = Model::new("test", "test");
        let users = QueryColumn {
            table_id: 0,
            column_id: 1,
            aggregation: Some(Aggregation::Sum),
            format: None,
            order: Some(Order::Asc),
            data_type: TypeAlias::Integer,
            table_name: String::from("orderdetails"),
            column_name: String::from("quantity"),
        };

        let builder = PostgresDialect {
            model,
            schema: "public".to_string(),
        };

        let query = builder.select_number(&users);
        assert_eq!(
            "ROUND (SUM(\"public\".\"orderdetails\".\"quantity\")::numeric, 4)::float as \"quantity\"",
            &query.to_string()
        );
    }

    #[test]
    fn select_should_work_for_formatted_dates() {
        let model = Model::new("test", "test");
        let orders = QueryColumn {
            table_id: 0,
            column_id: 1,
            table_name: String::from("orders"),
            column_name: String::from("orderdate"),
            aggregation: None,
            format: Some(Format::Quarter),
            order: Some(Order::Asc),
            data_type: TypeAlias::Date,
        };

        let builder = PostgresDialect {
            model,
            schema: "public".to_string(),
        };

        let query = builder.select_date(&orders);
        assert_eq!(
            "to_char(\"public\".\"orders\".\"orderdate\", 'YYYY-\"Q\"Q') as \"orderdate\"",
            &query.to_string()
        );
    }
}
