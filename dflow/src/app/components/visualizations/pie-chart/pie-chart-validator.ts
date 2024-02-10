import { QueryResponse } from "../../user-query/services/query";
import { VisualizationValidator } from "../types";

export const PieChartValidator: VisualizationValidator = (data: QueryResponse) => {
    return false
};
