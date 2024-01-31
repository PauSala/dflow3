'use client'

import { Column, Table } from "../../../model/data-model"
import { AggregationValue } from "../model/user-query";

export interface JoinModulesState {
    id: string;
    mainTable: Table,
    mainTableColumns: Column[],
    joinDefinition: {
        mainTableColumn: Column,
        joinTable: Table,
        joinColumn: Column
    }
}[];

export interface AggregationModulesState {
    id: string;
    table: Table;
    column: Column;
    aggregation: AggregationValue;
}

export interface UserQueryState {
    mainTable: Table;
    mainTableColumns: Column[];
    joinModules: JoinModulesState[];
    aggregationModules: AggregationModulesState[]
}

