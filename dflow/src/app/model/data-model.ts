export interface Column {
    column_id: number;
    name: string;
    display_name: string;
    type_alias: "Number" | "Text" | "Date";
    actual_type: string;
}

export interface Table {
    table_id: number;
    name: string;
    display_name: string;
    columns: Record<string, Column>;
    column_count: number;
    relations: number[];
}

export interface Relation {
    id: number;
    pk_table: number;
    fk_table: number;
    pk_column: number;
    fk_column: number;
    active: boolean;
}

export interface DataModel {
    datasource_id: string;
    id: string;
    tables: Record<string, Table>;
    relations: Record<string, Relation>;
    relation_count: number;
}
