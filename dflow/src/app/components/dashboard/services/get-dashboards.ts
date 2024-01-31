import axios from "axios"
import { DashboardDto } from "./post-dashboard"


export const getDashboardsByUser = async (userId: string = "test"): Promise<DashboardDto[]> => {
    let response = await axios.get(`http://127.0.0.1:8000/dashboard/${userId}`)
    return response.data
}
