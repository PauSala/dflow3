"use-client";
import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../../../model/user-query";
import { ChartType } from "../../visualizations/types";
import { LineChartWrapper } from "../../visualizations/line-chart/line-chart";
import { QueryResponse, query } from "../../user-query/services/query";
import BarChartWrapper from "../../visualizations/bar-chart/bar-chart";
import { ChartRenderer } from "../../visualizations/chart-renderer";

export type PanelContentType = {
  type: "chart";
  chartType?: ChartType;
};

export interface PanelProps {
  builder: UserQueryBuilder;
  name: string;
  id: string;
  content: PanelContentType;
  width: number;
  height: number;
}

export default function Panel({
  id,
  name,
  builder,
  content,
  width,
  height,
}: PanelProps) {
  const style = { width: `${width}px`, height: `${height}px` };
  const [data, setData] = useState<QueryResponse>({ columns: [], data: [] });
  const [loading, setLoading] = useState(true);
  const userQuery = builder.build();
  useEffect(() => {
    const getData = async () => {
      let data = await query(userQuery);
      setData(data);
    };
    getData().then(() => setLoading(false));
  }, [builder]);

  return (
    <div style={style}>
      <p>{content.chartType}</p>
      {!loading && (
        <ChartRenderer
          chartProps={{ chartData: { data, userQuery } }}
          chartType={content.chartType!}
        ></ChartRenderer>
      )}
    </div>
  );
}
