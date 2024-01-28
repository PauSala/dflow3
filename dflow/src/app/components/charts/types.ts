'use-client'
import { UserQuery } from "../../model/user-query";
import { QueryResponse } from "../user-query/services/query";
import { BarChartValidator } from "./bar-chart/bar-chart-validator";
import { LineChartValidator } from "./line-chart/line-chart-validator";
import { PieChartValidator } from "./pie-chart/pie-chart-validator";

export interface ChartProps {
    userQuery: UserQuery;
    data: QueryResponse;
}

export type ChartType = "line" | "bar" | "hBar" | "pie" | "table";
export interface Chart {
    type: ChartType;
    validator: (data: UserQuery) => boolean
}

export type ChartValidator = (data: UserQuery) => boolean;


export function chartValidatorProvider(t: ChartType): ChartValidator {
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
