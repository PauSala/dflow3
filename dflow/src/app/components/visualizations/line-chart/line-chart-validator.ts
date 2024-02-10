'use-client'
import { QueryResponse } from "../../user-query/services/query";
import { VisualizationValidator } from "../types";

export const LineChartValidator: VisualizationValidator = (data: QueryResponse) => {
    return false;
};
