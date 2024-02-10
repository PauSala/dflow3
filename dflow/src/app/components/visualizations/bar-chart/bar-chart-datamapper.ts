import { ChartDataMapper, GraphicableData } from "../../user-query/services/data-mapping/mappers";

export interface BarChartData {
    numericalFields: Array<string>;
    categoricalField: string;
    values: Record<string, string | number>[];
}

export const barChartDataMapper: ChartDataMapper<BarChartData> = (i: GraphicableData) => {
    if (i.categorycalFields.length > 2) {
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
