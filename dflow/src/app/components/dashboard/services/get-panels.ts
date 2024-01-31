import axios from "axios"
import { PanelDto } from "./post-panel"

export const getPanel = async (panelId: string = "928f9578-752a-4469-9e77-2da3a40f4933"): Promise<PanelDto> => {
    let response = await axios.get(`http://127.0.0.1:8000/dashboard/panel/${panelId}`)
    return response.data
}
