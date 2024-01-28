import React from "react";
import { ChartProps, ChartType } from "./types";
import BarChartWrapper from "./bar-chart/bar-chart";
import { LineChartWrapper } from "./line-chart/line-chart";

export interface ChartWrapperProps {
  chartData: ChartProps;
}

export const ChartRenderer = ({
  chartType,
  chartProps,
}: {
  chartType: ChartType;
  chartProps: ChartWrapperProps;
}) => {
  let ChartComponent;
  switch (chartType) {
    case "bar":
      ChartComponent = BarChartWrapper;
      break;
    case "line":
      ChartComponent = LineChartWrapper;
      break;
    default:
      ChartComponent = () => <div>Not implemented yet!</div>;
  }

  // Render the determined component
  return <ChartComponent chartData={chartProps.chartData} />;
};
