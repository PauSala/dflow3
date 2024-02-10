'use-client'
import { QueryResponse } from "../../user-query/services/query";
import { VisualizationValidator } from "../types";

export const BarChartValidator: VisualizationValidator = (data: QueryResponse) => {
    let isValid = true;
    if (data.count_categorical > 2) {
        isValid = false;
    }
    if (data.count_numerical < 1) {
        isValid = false;
    }

    return isValid
};
