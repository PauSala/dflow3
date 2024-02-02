'use-client'
import { UserQuery } from "../user-query/model/user-query";
import { QueryResponse } from "../user-query/services/query";
import { BarChartValidator } from "./bar-chart/bar-chart-validator";
import { LineChartValidator } from "./line-chart/line-chart-validator";
import { PieChartValidator } from "./pie-chart/pie-chart-validator";

export interface VisualizationProps {
    userQuery: UserQuery;
    data: QueryResponse;
}

export type VisualizationType = "line" | "bar" | "hBar" | "pie" | "table";
export interface Chart {
    type: VisualizationType;
    validator: (data: UserQuery) => boolean
}

export type VisualizationValidator = (data: UserQuery) => boolean;


export function visualizationValidatorProvider(t: VisualizationType): VisualizationValidator {
    switch (t) {
        case "line":
            return LineChartValidator;
        case "bar":
            return BarChartValidator;
        case "hBar":
            return BarChartValidator;
        case "pie":
            return PieChartValidator;
        case "table":
            return () => true;
    }
}
