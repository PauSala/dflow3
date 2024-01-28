"use-client";
import React from "react";
import { ChartProps } from "../types";
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

export interface BarChartWrapperProps {
  chartData: ChartProps;
}

export default function BarChartWrapper({ chartData }: BarChartWrapperProps) {
  const mapped: BarChartData = barChartDataMapper(
    queryToGraphicable({ q: chartData.userQuery, r: chartData.data })
  );
  return (
    <ResponsiveContainer width="85%" height="85%">
      <BarChart
        width={500}
        height={300}
        data={mapped.values}
        margin={{
          top: 50,
          right: 0,
          left: 0,
          bottom: 0,
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
        <ReferenceLine y={0} stroke="#000" />
        {mapped.numericalFields.map((col) => (
          <Bar
            key={col}
            type="monotone"
            dataKey={col}
            fill="#8884d8"
          />
        ))}
      </BarChart>
    </ResponsiveContainer>
  );
}
