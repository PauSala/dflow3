import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../../../model/user-query";
import { ChartType } from "../../charts/types";
import { LineChartWrapper } from "../../charts/line-chart/line-chart";
import { QueryResponse, query } from "../../user-query/services/query";

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
  const userQuery = builder.build();
  useEffect(() => {
    const getData = async () => {
      let data = await query(userQuery);
      setData(data);
    }
    getData();
  }, [builder])

  return (
    <div style={style}>
      <p>{name}</p>
      <LineChartWrapper userQuery={userQuery} data={data}></LineChartWrapper>
    </div>
  );
}
