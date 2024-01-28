import axios from "axios";
import { UserQuery } from "../../../model/user-query";

export interface QueryResponse { columns: Array<string>, data: Array<Array<number | string>> }

export const query = async (query: UserQuery): Promise<QueryResponse> => {
    return axios.post('http://127.0.0.1:8000/query',
        { ...query }
    ).then(response => response.data as QueryResponse)
}
