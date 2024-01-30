'use client'
import { v4 } from "uuid";
import { AggregationModulesState, JoinModulesState, UserQueryState } from "../components/user-query/services/query-from-builder";
import { Column, DataModel, Table } from "./data-model";

export type AggregationValue = null | "Sum" | "Min" | "Max" | "Avg" | "Count" | "CountDistinct"

export type Format =
    | null
    | "Year"
    | "Quarter"
    | "Month"
    | "Week"
    | "Day"
    | "DayHour"
    | "DayHourMinute"
    | "Timestamp"
    | "WeekDay"

export type TypeAlias = "Integer" | "Float" | "Text" | "Date";
export type Order = null | "Asc" | "Desc";
export interface QueryColumn {
    table_id: number;
    column_id: number;
    table_name: string;
    column_name: string;
    aggregation: AggregationValue;
    format: Format;
    order: "Asc" | "Desc" | null;
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

export interface JoinDefinition {
    main_table_id: number;
    join_table_id: number;
    main_field_id: number;
    join_field_id: number;
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
        joins: JoinDefinition[]
    }
}

type Primitives = string | number | boolean | symbol | null | undefined;

type NonShallowCopyableObject<T> = { [K in keyof T]: T[K] extends Primitives ? T[K] : never };

export class UserQueryBuilder {

    private tables: Map<number, QueryColumn[]> = new Map();
    private joins: Map<string, JoinDefinition> = new Map();


    constructor(
        private model: DataModel,
        private datasource_id: string,
        private model_id: string,
    ) { }

    public getModel() {
        return this.model;
    }

    public getJoins() {
        return this.joins;
    }

    public getAggregations() {
        return []
    }

    private cloneMap<T extends NonShallowCopyableObject<T>>(map: Map<unknown, T[]> | Map<unknown, T>) {
        let newMap = new Map();
        if (typeof map)
            map.forEach((value, key) => {
                if (Array.isArray(value)) {
                    let newA = value.map(v => ({ ...v }));
                    newMap.set(key, newA);
                }
                else {
                    newMap.set(key, { ...value })
                }
            });
        return newMap;
    }

    public newInstance(): UserQueryBuilder {
        const newInstance = new UserQueryBuilder(this.model, this.datasource_id, this.model_id);
        newInstance.tables = this.cloneMap(this.tables);
        newInstance.joins = this.cloneMap(this.joins);
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

    public getMainTable() {
        const mapIterator = this.tables.entries();
        const firstElement: [number, QueryColumn[]] | undefined = mapIterator.next().value;
        if (firstElement) {
            return this.model.tables[firstElement[0]];
        }
        return firstElement
    }

    public getColumns(table_id: number) {
        return this.tables.get(table_id);
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
                joins: Array.from(this.joins.values())
            }
        }
        return query;
    }

    public addJoin(joinDefinition: JoinDefinition, key: string) {
        this.joins.set(key, joinDefinition);
    }

    public removeJoin(key: string) {
        this.joins.delete(key);
    }

    public getSumarizableColumns(): { column: Column & { agg: AggregationValue }; table: Table }[] {
        const response: { column: Column & { agg: AggregationValue }; table: Table }[] = [];
        this.tables.forEach((cols, table_id) => {
            const table = this.model.tables[table_id];
            const columns = cols
                .filter(c => c.data_type == "Integer" || c.data_type == "Float")
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
            format: null,
            order: null,
            data_type: column.type_alias
        });
    }

    public userQueryState(): UserQueryState | undefined {
        //Set main table info
        const mainTable = this.getMainTable();
        if (!mainTable) {
            return undefined
        }
        const mainTableColumns = this.getColumns(mainTable.table_id)
            ?.map(c => mainTable.columns[c.column_id]) || [];

        //Set joins info
        const joinModules: JoinModulesState[] = [];
        this.joins.forEach((value, key) => {
            const mainTable = this.model.tables[value.main_table_id];
            joinModules.push({
                id: key,
                mainTable: mainTable,
                mainTableColumns:
                    this.getColumns(value.main_table_id)
                        ?.map(c => mainTable.columns[c.column_id]) || [],
                joinDefinition: {
                    mainTableColumn: mainTable.columns[value.main_field_id],
                    joinTable: this.model.tables[value.join_table_id],
                    joinColumn: this.model.tables[value.join_table_id].columns[value.join_field_id]
                }
            });
        });

        //Set aggregation info
        const aggregationModules: AggregationModulesState[] = [];
        this.tables.forEach((columns, key) => {
            columns.forEach(col => {
                if(col.aggregation !== null){
                    aggregationModules.push(
                        {
                            id: v4(),
                            table: this.model.tables[key],
                            column: this.model.tables[key].columns[col.column_id],
                            aggregation: col.aggregation
                        }
                    )
                }
            });
        })

        return {
            mainTable,
            mainTableColumns,
            joinModules,
            aggregationModules: aggregationModules
        }
    }
}
