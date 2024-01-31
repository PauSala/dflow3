import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../model/user-query";
import ChartSelector from "../selectors/charts/chart-selector";
import { ChartType, chartValidatorProvider } from "../../visualizations/types";
import { ChartRenderer } from "../../visualizations/chart-renderer";
import { QueryResponse, postQuery } from "../services/query";

export type DrawableChartsState = {
  [T in ChartType]: { enabled: boolean; name: T };
};

const defaultDrawableChartState: DrawableChartsState = {
  line: { name: "line", enabled: false },
  pie: { name: "pie", enabled: false },
  bar: { name: "bar", enabled: false },
  table: { name: "table", enabled: true },
  hBar: { name: "hBar", enabled: false },
};

export default function Visualization({
  queryBuilder,
  onChartType,
}: {
  queryBuilder: UserQueryBuilder;
  onChartType: (ct: ChartType) => void;
}) {
  const [validated, setValidated] = useState<DrawableChartsState>(
    defaultDrawableChartState
  );
  const [chartType, setChartType] = useState<ChartType>("table");
  const [data, setData] = useState<QueryResponse>({ columns: [], data: [] });

  useEffect(() => {
    const userQuery = queryBuilder.build();
    setValidated(() => {
      return {
        bar: { enabled: chartValidatorProvider("bar")(userQuery), name: "bar" },
        hBar: {
          enabled: chartValidatorProvider("hBar")(userQuery),
          name: "hBar",
        },
        line: {
          enabled: chartValidatorProvider("line")(userQuery),
          name: "line",
        },
        pie: { enabled: chartValidatorProvider("pie")(userQuery), name: "pie" },
        table: {
          enabled: chartValidatorProvider("table")(userQuery),
          name: "table",
        },
      };
    });
    const getData = async () => {
      let data = await postQuery(userQuery);
      setData(data);
    };
    getData();
  }, [queryBuilder]);

  const onChartChange = async (ct: ChartType) => {
    setChartType(ct);
    onChartType(ct);
  };

  return (
    <div className="flex flex-col  min-h-80">
      <ChartSelector
        validated={validated}
        onChange={onChartChange}
      ></ChartSelector>
      <div className="h-[50vh] flex items-center">
        <ChartRenderer
          chartType={chartType}
          chartProps={{
            chartData: { data, userQuery: queryBuilder.build() },
          }}
        ></ChartRenderer>
      </div>
    </div>
  );
}
