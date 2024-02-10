"use client";
import React from "react";
import { queryToGraphicable } from "../../user-query/services/data-mapping/mappers";
import { BarChartData, barChartDataMapper } from "./bar-chart-datamapper";
import {
  ResponsiveContainer,
  CartesianGrid,
  XAxis,
  YAxis,
  Tooltip,
  Legend,
  BarChart,
  ReferenceLine,
  Bar,
} from "recharts";
import { VisualizationWrapperProps } from "../chart-renderer";
import { defaultColorPalette } from "../../../theme/chart-palette";


export default function BarChartWrapper({ chartData }: VisualizationWrapperProps) {
  const mapped = barChartDataMapper(
    queryToGraphicable({ q: chartData.userQuery, r: chartData.data })
  );
  return (
    <ResponsiveContainer width="95%" height="90%">
      <BarChart
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

        {/* Issue with this: https://github.com/recharts/recharts/issues/3615
          <ReferenceLine y={0} stroke="#000" /> */}
        {mapped.numericalFields.map((col, i) => (
          <Bar key={col} type="monotone" dataKey={col} fill={defaultColorPalette[i]} />
        ))}
      </BarChart>
    </ResponsiveContainer>
  );
}
