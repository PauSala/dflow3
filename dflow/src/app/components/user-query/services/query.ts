import axios from "axios";
import { QueryColumn, UserQuery } from "../model/user-query";
export interface ShapedField {
    field: QueryColumn;
    label: string;
    values: Array<string | number>
}
export interface QueryResponse {
    numerical_fields: Array<ShapedField>,
    categorical_fields: Array<ShapedField>,
    count_numerical: number;
    count_categorical: number;
}

export const postQuery = async (query: UserQuery): Promise<QueryResponse> => {
    return axios.post('http://127.0.0.1:8000/query',
        { ...query }
    ).then(response => response.data as QueryResponse)
}
