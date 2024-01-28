import { UserQuery } from "../../model/user-query";
import { LineChartValidator } from "./line-chart/line-chart-validator";

export type ChartType = "line" | "bar" | "hBar" | "pie";
export interface Chart {
    type: ChartType;
    validator: (data: UserQuery) => boolean
}

export type ChartValidator = (data: UserQuery) => boolean;


export function ChartValidatorProvider(t: ChartType): ChartValidator {
    switch (t) {
        case "line":
            return LineChartValidator;
        case "bar":
            throw Error("Not implemented")
        case "hBar":
            throw Error("Not implemented")
        case "pie":
            throw Error("Not implemented")
    }
}
