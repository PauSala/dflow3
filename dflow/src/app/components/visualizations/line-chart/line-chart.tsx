"use-client";
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
import { UserQuery } from "../../../model/user-query";
import { QueryResponse } from "../../user-query/services/query";
import { LineChartData, lineChartDataMapper } from "./line-chart-datamapper";
import { queryToGraphicable } from "../../user-query/services/data-mapping/mappers";
import { ChartProps } from "../types";
import { ChartWrapperProps } from "../chart-renderer";

export interface LineChartWrapperProps {
  chartData: ChartProps;
}


export function LineChartWrapper({ chartData }: ChartWrapperProps) {
  const mapped: LineChartData = lineChartDataMapper(queryToGraphicable({q: chartData.userQuery, r: chartData.data}));
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
        <XAxis dataKey={mapped.categoricalField} tick={{ fontSize: "0.7em",  }}  tickLine={false} axisLine={false} />
        <YAxis tick={{ fontSize: "0.7em" }} axisLine={false}  tickLine={false} />
        <Tooltip />
        <Legend />
        {mapped.numericalFields.map((col) => (
          <Line
            key={col}
            type="monotone"
            dataKey={col}
            stroke="#8884d8"
            strokeWidth={2}
            activeDot={{ r: 3 }}
            dot={false}
          />
        ))}
      </LineChart>
    </ResponsiveContainer>
  );
}
