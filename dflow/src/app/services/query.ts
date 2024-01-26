import axios from "axios";
import { Column, Table } from "../model/data-model";
import { UserQuery } from "../model/user-query";

export const query = async (query: UserQuery) => {
    return axios.post('http://127.0.0.1:8000/query',
        {...query}
    ).then(response => response.data as { columns: Array<string>, data: Array<Array<number | string>> })
}

function checkSummarize(summarize: {
    column: Column;
    table: Table;
    aggregation: string;
},
    column: { column: Column, table: Table }) {
    return summarize.column.column_id === column.column.column_id && summarize.table.table_id === column.table.table_id
        ? summarize.aggregation
        : null;
}

export const userQuery = async (
    columns: Array<{ column: Column, table: Table }>,
    summarize: {
        column: Column;
        table: Table;
        aggregation: string;
    }) => {

    return axios.post('http://127.0.0.1:8000/query',
        {
            datasource_id: "mssql",
            query: {
                columns: columns.map(c => ({
                    table_id: c.table.table_id,
                    column_id: c.column.column_id,
                    table_name: c.table.name,
                    column_name: c.column.name,
                    aggregation: checkSummarize(summarize, c),
                    format: "No",
                    order: "No",
                    data_type: c.column.type_alias,
                })),
                model_id: "InformaHUD",
                filters: []
            }
        }
    ).then(response => response.data as { columns: Array<string>, data: Array<Array<number | string>> })
}
