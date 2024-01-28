import { UserQuery } from "../../../model/user-query";
import { numericalFieldsFromUserQuery, categorycalFieldsFromUserQuery } from "../../user-query/services/data-mapping/mappers";
import { ChartValidator } from "../types";

export const PieChartValidator: ChartValidator = (data: UserQuery) => {
    let isValid = true;
    let numericFields = numericalFieldsFromUserQuery(data);
    let categoricalFields = categorycalFieldsFromUserQuery(data);

    if (numericFields.length !== 1) {
        isValid = false;
    }
    if (categoricalFields.length !== 1) {
        isValid = false;
    }

    return isValid
};