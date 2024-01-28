'use-client'
import { ChartDataMapper, GraphicableData } from "../../user-query/services/data-mapping/mappers";

export interface LineChartData {
    numericalFields: Array<string>;
    categoricalField: string;
    values: Record<string, string | number>[];
}

export const lineChartDataMapper: ChartDataMapper<LineChartData> = (i: GraphicableData) => {
    if (i.categorycalFields.length !== 1) {
        throw new Error("Line chart does not allow more thant one categorycal field");
    }
    if (i.numericalFields.length < 1) {
        throw new Error("Line chart does not allow less thant one numerical field");
    }
    return {
        values: i.values,
        categoricalField: i.categorycalFields[0],
        numericalFields: i.numericalFields
    }

}
