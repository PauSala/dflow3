use crate::modules::{
    dmodel::model::model::{Model, TypeAlias},
    query::model::query_builder::abstract_query::{
        AbstractQuery, Aggregation, FilterValue, Format, Operator, QueryColumn, QueryFilter,
    },
};

use super::SqlBuilderDialect;

pub struct MssqlDialect {
    pub model: Model,
    pub schema: String,
}

impl SqlBuilderDialect for MssqlDialect {
    //TODO: makes sense to have some count(format(date))?

    fn date_format(&self, c: &QueryColumn) -> String {
        let format;
        match c.format {
            Some(Format::Year) => format = String::from("'yyyy'"),
            Some(Format::Quarter) => format = String::from("'yyyy-Q'"),
            Some(Format::Month) => format = String::from("'yyyy-MM'"),
            Some(Format::Week) => {
                return format!(
                    "DATEPART(week, CAST(\"{}\".\"{}\".\"{}\" AS DATE)) as \"{}\"",
                    self.schema, c.table_name, c.column_name, c.column_name
                )
                .to_string();
            }
            Some(Format::WeekDay) => {
                return format!(
                    "DATEPART(weekday, CAST(\"{}\".\"{}\".\"{}\" AS DATE)) as \"{}\"",
                    self.schema, c.table_name, c.column_name, c.column_name
                )
                .to_string();
            }
            Some(Format::Day) => format = String::from("'yyyy-MM-dd'"),
            Some(Format::DayHour) => format = String::from("'yyyy-MM-dd HH"),
            Some(Format::DayHourMinute) => format = String::from("'yyyy-MM-dd HH:mm'"),
            Some(Format::Timestamp) => format = String::from("'yyyy-MM-dd HH:mm:ss"),
            None => format = String::from("'yyyy-MM-dd HH:mm:ss'"),
        }
        format
    }

    fn select_date(&self, c: &QueryColumn) -> String {
        let format = self.date_format(c);
        format!(
            "FORMAT(CAST(\"{}\".\"{}\".\"{}\" AS DATE), {}) as \"{}\"",
            self.schema, c.table_name, c.column_name, format, c.column_name
        )
        .to_string()
    }

    fn select_number(&self, c: &QueryColumn) -> String {
        let content;
        match c.aggregation {
            Some(agg) => match agg {
                Aggregation::CountDistinct => {
                    content = format!(
                        "CAST ({} (DISTINCT \"{}\".\"{}\".\"{}\") AS DECIMAL(32, 4)) as \"{}\"",
                        agg.get_value(),
                        self.schema,
                        c.table_name,
                        c.column_name,
                        c.column_name
                    )
                }
                _ => {
                    content = format!(
                        "CAST ({}(\"{}\".\"{}\".\"{}\") AS DECIMAL(32, 4)) as \"{}\"",
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
                    "CAST (\"{}\".\"{}\".\"{}\" AS DECIMAL(32, 4)) as \"{}\"",
                    self.schema, c.table_name, c.column_name, c.column_name
                )
            }
        }
        content
    }

    fn select_text(&self, c: &QueryColumn) -> String {
        //TODO: add count and count distinct
        format!(
            "\"{}\".\"{}\".\"{}\" as \"{}\"",
            self.schema, c.table_name, c.column_name, c.column_name
        )
    }

    fn from_clause(
        &self,
        c: &QueryColumn,
        s: &sql_query_builder::Select,
    ) -> sql_query_builder::Select {
        s.clone().from(&format!("\"{}\"", c.table_name))
    }

    fn group_by(
        &self,
        mut select: sql_query_builder::Select,
        query: &AbstractQuery,
    ) -> sql_query_builder::Select {
        for col in query
            .columns
            .iter()
            .filter(|c| c.aggregation.is_none())
            .collect::<Vec<_>>()
            .iter()
        {
            match col.data_type {
                TypeAlias::Date => {
                    select = select.group_by(&format!(
                        "FORMAT(CAST(\"{}\".\"{}\".\"{}\" AS DATE), {})",
                        self.schema,
                        col.table_name,
                        col.column_name,
                        &self.date_format(&col),
                    ))
                }
                _ => {
                    select = select.group_by(&format!(
                        "\"{}\".\"{}\".\"{}\"",
                        self.schema, col.table_name, col.column_name
                    ));
                }
            }
        }
        select
    }

    fn order_by(
        &self,
        mut select: sql_query_builder::Select,
        query: &AbstractQuery,
    ) -> sql_query_builder::Select {
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
                col.order.as_ref().expect("Should be filtered").get_value()
            ));
        }
        select
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
                            panic!("This vector should contain only Int or DECIMAL")
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

    fn join(
        &self,
        mut select: sql_query_builder::Select,
        query: &AbstractQuery<'_>,
    ) -> sql_query_builder::Select {
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
                .get(&join.join_table_id)
                .expect("table should exist");
            let pk_column = &pk_table
                .columns
                .get(&join.main_field_id)
                .expect("Column should exist")
                .name;
            let fk_column = &fk_table
                .columns
                .get(&join.join_field_id)
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
}
