"use client";
import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";
import { LineChartData, lineChartDataMapper } from "./line-chart-datamapper";
import { queryToGraphicable } from "../../user-query/services/data-mapping/mappers";
import { VisualizationProps } from "../types";
import { VisualizationWrapperProps } from "../chart-renderer";
import { defaultColorPalette } from "../../../theme/chart-palette";

export interface LineChartWrapperProps {
  chartData: VisualizationProps;
}

export function LineChartWrapper({ chartData }: VisualizationWrapperProps) {
  const mapped: LineChartData = lineChartDataMapper(
    queryToGraphicable({ q: chartData.userQuery, r: chartData.data })
  );
  return (
    <ResponsiveContainer width="95%" height="90%">
      <LineChart
        width={500}
        height={300}
        data={mapped.values}
        margin={{
          top: 20,
          right: 20,
          left: 20,
          bottom: 20,
        }}
      >
        <CartesianGrid horizontal={true} vertical={false} />
        <XAxis
          dataKey={mapped.categoricalField}
          tick={{ fontSize: "0.7em" }}
          tickLine={false}
          axisLine={false}
        />
        <YAxis tick={{ fontSize: "0.7em" }} axisLine={false} tickLine={false} />
        <Tooltip />
        <Legend />
        {mapped.numericalFields.map((col, i) => (
          <Line
            key={col}
            type="monotone"
            dataKey={col}
            stroke={defaultColorPalette[i]}
            strokeWidth={2}
            activeDot={{ r: 3 }}
            dot={false}
          />
        ))}
      </LineChart>
    </ResponsiveContainer>
  );
}
