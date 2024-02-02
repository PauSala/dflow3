import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../model/user-query";
import ChartSelector from "../selectors/charts/chart-selector";
import { VisualizationType, visualizationValidatorProvider } from "../../visualizations/types";
import { VisualizationRenderer } from "../../visualizations/chart-renderer";
import { QueryResponse, postQuery } from "../services/query";

export type DrawableChartsState = {
  [T in VisualizationType]: { enabled: boolean; name: T };
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
  onChartType: (ct: VisualizationType) => void;
}) {
  const [validated, setValidated] = useState<DrawableChartsState>(
    defaultDrawableChartState
  );
  const [chartType, setChartType] = useState<VisualizationType>("table");
  const [data, setData] = useState<QueryResponse>({ columns: [], data: [] });

  useEffect(() => {
    const userQuery = queryBuilder.build();
    setValidated(() => {
      return {
        bar: { enabled: visualizationValidatorProvider("bar")(userQuery), name: "bar" },
        hBar: {
          enabled: visualizationValidatorProvider("hBar")(userQuery),
          name: "hBar",
        },
        line: {
          enabled: visualizationValidatorProvider("line")(userQuery),
          name: "line",
        },
        pie: { enabled: visualizationValidatorProvider("pie")(userQuery), name: "pie" },
        table: {
          enabled: visualizationValidatorProvider("table")(userQuery),
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

  const onChartChange = async (ct: VisualizationType) => {
    setChartType(ct);
    onChartType(ct);
  };

  return (
    <div className="flex flex-col  min-h-80">
      <ChartSelector
        validated={validated}
        onChange={onChartChange}
      ></ChartSelector>
      <div className="h-[50vh] flex items-center justify-center">
        <VisualizationRenderer
          chartType={chartType}
          visualizationProps={{
            chartData: { data, userQuery: queryBuilder.build() },
          }}
        ></VisualizationRenderer>
      </div>
    </div>
  );
}
