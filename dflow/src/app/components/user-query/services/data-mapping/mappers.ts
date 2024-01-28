import { UserQuery } from "../../../../model/user-query";
import { QueryResponse } from "../query";

export type DataMapper<T, U> = (i: T) => U;

export interface GraphicableData {
    numericalFields: string[];
    categorycalFields: string[];
    values: Record<string, number | string>[]
}
export interface QueryToGraphicableInput {
    q: UserQuery;
    r: QueryResponse;
}

export type ChartDataMapper<T> = DataMapper<GraphicableData, T>;

export function numericalFieldsFromUserQuery(userQuery: UserQuery): string[] {
    return userQuery.query.columns
        .filter(c => c.data_type === "Float" || c.data_type === "Integer")
        .map(c => c.column_name)
}
export function categorycalFieldsFromUserQuery(userQuery: UserQuery): string[] {
    return userQuery.query.columns
        .filter((c) => c.data_type === "Text" || c.data_type === "Date")
        .map(c => c.column_name);
}

export const queryToGraphicable: DataMapper<QueryToGraphicableInput, GraphicableData> =
    ({ q, r }: QueryToGraphicableInput) => {
        let numericalFields = numericalFieldsFromUserQuery(q);
        let categorycalFields = categorycalFieldsFromUserQuery(q);
        let columns = r.columns;
        let data = r.data.map((row) => {
            let parsedRow: Record<string, string | number> = {};
            row.forEach((item, index) => {
                parsedRow[columns[index]] = item;
            });
            return parsedRow;
        });
        return {
            values: data,
            numericalFields,
            categorycalFields
        }
    }

