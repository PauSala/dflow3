import axios from "axios"

export interface DashboardDto {
    id: string,
    user_id: string,
    model_id: string,
    name: string,
    config: {
        panels: string[]
    }
}


export const postDashboard = async (dashboardDto: DashboardDto): Promise<DashboardDto> => {
    return axios.post('http://127.0.0.1:8000/dashboard/save',
        { ...dashboardDto }
    );
}
