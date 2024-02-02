"use client";
import React from "react";
import { VisualizationProps, VisualizationType } from "./types";
import BarChartWrapper from "./bar-chart/bar-chart";
import { LineChartWrapper } from "./line-chart/line-chart";
import { DFlowTable } from "./table/dflow-table";

export interface VisualizationWrapperProps {
  chartData: VisualizationProps;
}

export function VisualizationRenderer({
  chartType,
  visualizationProps,
}: {
  chartType: VisualizationType;
  visualizationProps: VisualizationWrapperProps;
}) {
  let ChartComponent;
  switch (chartType) {
    case "bar":
      ChartComponent = BarChartWrapper;
      break;
    case "line":
      ChartComponent = LineChartWrapper;
      break;
    case "table":
      ChartComponent = DFlowTable;
      break;
    default:
      ChartComponent = function Default() {
        return <div>Not implemented yet!</div>;
      };
  }

  // Render the determined component
  return <ChartComponent chartData={visualizationProps.chartData} />;
}
