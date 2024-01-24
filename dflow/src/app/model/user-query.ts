import { Column, DataModel, Table } from "./data-model";

export type AggregationValue = null | "Sum" | "Min" | "Max" | "Avg" | "Count" | "CountDistinct"

export type Format =
    | "No"
    | "Year"
    | "Quarter"
    | "Month"
    | "Week"
    | "Day"
    | "DayHour"
    | "DayHourMinute"
    | "Timestamp"
    | "WeekDay"

export type TypeAlias = "Number" | "Text" | "Date";
export type Order = "Asc" | "Desc" | "No";
export interface QueryColumn {
    table_id: number;
    column_id: number;
    table_name: string;
    column_name: string;
    aggregation: AggregationValue;
    format: Format;
    order: "Asc" | "Desc" | "No";
    data_type: TypeAlias;
}

export type Operator =
    | "Eq"
    | "Ge"
    | "G"
    | "Se"
    | "Sm"
    | "NotEq"
    | "Like"
    | "In"
    | "Between";

export type FilterValue =
    | {
        Number: number
    }
    | {
        Text: string
    }
    | {
        Date: string
    }
export type ValueContainer =
    | {
        UniValue: FilterValue
    }
    | {
        MultiValue: FilterValue[]
    }

export interface QueryFilter {
    column_name: string,
    table_name: string,
    column_id: number,
    table_id: number,
    operator: Operator,
    value: ValueContainer,
    data_type: TypeAlias,
}
export interface UserQuery {
    datasource_id: string,
    query: {
        columns: QueryColumn[],
        model_id: string,
        filters: QueryFilter[]
    }
}

export class UserQueryBuilder {

    private tables: Map<number, QueryColumn[]> = new Map();
    constructor(
        private model: DataModel,
        private datasource_id: string,
        private model_id: string,
    ) { }

    public newInstance(): UserQueryBuilder {
        const newInstance = new UserQueryBuilder(this.model, this.datasource_id, this.model_id);
        newInstance.tables = this.tables;
        return newInstance;
    }

    public addColumnsToTable(table_id: number, columns: number[]) {
        let cols = columns.map(c => this.buildColumn(c, table_id));
        if (!this.tables.has(table_id)) {
            this.tables
                .set(table_id, cols);
        } else {
            let node = this.tables.get(table_id);
            node = cols;
            this.tables.set(table_id, node);
        }
    }

    public deleteTable(table_id: number) {
        this.tables.delete(table_id);
    }

    public build(): UserQuery {
        let columns: QueryColumn[] = [];
        this.tables.forEach((cols, _) => {
            cols.forEach(c => columns.push(c));
        })
        let query: UserQuery = {
            datasource_id: this.datasource_id,
            query: {
                model_id: this.model_id,
                columns,
                filters: [],
            }
        }
        return query;
    }

    public getSumarizableColumns(): { column: Column & { agg: AggregationValue }; table: Table }[] {
        const response: { column: Column & { agg: AggregationValue }; table: Table }[] = [];
        this.tables.forEach((cols, table_id) => {
            const table = this.model.tables[table_id];
            const columns = cols
                .filter(c => c.data_type == "Number")
                .map(c => ({ ...table.columns[c.column_id], agg: c.aggregation }));
            columns.forEach(c => {
                response.push({ column: { ...c }, table: table });
            })
        });
        return response;
    }

    public addAggregation(table_id: number, column_id: number, aggregation: AggregationValue) {
        const columns = this.tables.get(table_id);
        if (!columns) {
            throw Error("Table not found");
        }
        const column = columns.find(c => c.column_id === column_id);
        if (!column) {
            throw Error("Column not found");
        }
        column.aggregation = aggregation;
    }

    public removeAggregation(table_id: number, column_id: number) {
        const columns = this.tables.get(table_id);
        if (!columns) {
            throw Error("Table not found");
        }
        const column = columns.find(c => c.column_id === column_id);
        if (!column) {
            throw Error("Column not found");
        }
        column.aggregation = null;
    }

    private buildColumn(column_id: number, table_id: number): QueryColumn {
        let table = this.model.tables[table_id];
        if (!table) {
            throw Error("Table not found");
        }
        const column = table.columns[column_id];
        if (!column) {
            throw Error("Column not found");
        }
        return ({
            table_id: table.table_id,
            column_id: column.column_id,
            table_name: table.name,
            column_name: column.name,
            aggregation: null,
            format: "No",
            order: "No",
            data_type: column.type_alias
        });
    }
}
