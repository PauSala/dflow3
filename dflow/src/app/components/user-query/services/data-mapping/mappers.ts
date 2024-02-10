'use client'
import { UserQuery } from "../../model/user-query";
import { QueryResponse, ShapedField } from "../query";

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

const zipArrays = (arrays: (number | string)[][]) => {
    if (arrays.length === 0) {
        return [];
    }
    let size = arrays[0].length;
    for (let i = 0; i > arrays.length; i++) {
        if (arrays[i].length !== size) {
            throw Error("Can't zip arrays of distinct sizes");
        }
    }
    const zipped: Array<(number | string)[]> = [];
    for (let i = 0; i < size; i++) {
        const inner: (number | string)[] = [];
        for (let j = 0; j < arrays.length; j++) {
            inner.push(arrays[j][i]);
        }
        zipped.push(inner);
    }
    return zipped;
}

export function queryToLabelsValues(r: QueryResponse) {
    const values: (string | number)[][] = [];
    const labels: string[] = [];
    r.numerical_fields.forEach(f => (values.push(f.values), labels.push(f.label)));
    r.categorical_fields.forEach(f => (values.push(f.values), labels.push(f.label)));
    return { labels, values: zipArrays(values) };
}

interface LabelsValues {
    labels: string[];
    values: (string | number)[][];
}

function intoObjects(input: LabelsValues) {
    let data: Record<string, number | string>[] = [];
    for (let i = 0; i < input.values.length; i++) {
        let obj: Record<string, number | string> = {};
        for (let j = 0; j < input.values[i].length; j++) {
            obj[input.labels[j]] = input.values[i][j];
        }
        data.push(obj);
    }
    return data;
}

export const queryToGraphicable: DataMapper<QueryToGraphicableInput, GraphicableData> =
    ({ q, r }: QueryToGraphicableInput) => {

        let numericalFields: string[] = r.numerical_fields.map(f => f.label);
        let categorycalFields: string[] = r.categorical_fields.map(f => f.label);
        let data: Record<string, number | string>[] = intoObjects(queryToLabelsValues(r));
        return {
            values: data,
            numericalFields,
            categorycalFields
        }
    }

