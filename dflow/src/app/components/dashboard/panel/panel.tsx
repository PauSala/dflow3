"use-client";
import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../../../model/user-query";
import { ChartType } from "../../visualizations/types";
import { QueryResponse, query } from "../../user-query/services/query";
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

  useEffect(() => {
    const userQuery = builder.build();
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
          chartProps={{ chartData: { data, userQuery: builder.build() } }}
          chartType={content.chartType!}
        ></ChartRenderer>
      )}
    </div>
  );
}
