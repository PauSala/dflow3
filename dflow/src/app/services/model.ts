import axios, { AxiosResponse } from "axios"
import { DataModel } from "../model/data-model"

export const getModel = async (modelId: string = "test"): Promise<DataModel> => {
    let response = await axios.get(`http://127.0.0.1:8000/model/${modelId}`)
    return response.data
}
