"use client";
import React, { useEffect, useState } from "react";
import { UserQueryBuilder } from "../../user-query/model/user-query";
import { VisualizationType } from "../../visualizations/types";
import { QueryResponse, postQuery } from "../../user-query/services/query";
import { VisualizationRenderer } from "../../visualizations/chart-renderer";
import PanelConfiguration from "./panel-configuration/panel-configuration";
import PanelMenu from "./panel-menu";

export type PanelContentType = {
  type: "Chart";
  chartType?: VisualizationType;
};

export interface PanelProps {
  builder: UserQueryBuilder;
  name: string;
  id: string;
  content: PanelContentType;
  width: number;
  height: number;
  resizing: boolean;
  onContentChange: (
    builder: UserQueryBuilder,
    chartType: VisualizationType,
    panelId: string
  ) => void;
  onDelete: (panelId: string) => void;
}

export default function Panel({
  id,
  name,
  builder,
  content,
  width,
  height,
  resizing,
  onContentChange,
  onDelete,
}: PanelProps) {
  const style = { width: `${width}px`, height: `${height}px` };
  const [data, setData] = useState<QueryResponse>({
    categorical_fields: [],
    numerical_fields: [],
    count_categorical: 0,
    count_numerical: 0,
  });
  const [loading, setLoading] = useState(true);
  const [openQueryConfig, setOpenQueryConfig] = useState(false);

  const onChange = (builder: UserQueryBuilder, chartType: VisualizationType) =>
    onContentChange(builder, chartType, id);

  useEffect(() => {
    const userQuery = builder.build();
    const getData = async () => {
      let data = await postQuery(userQuery);
      setData(data);
    };
    getData().then(() => setLoading(false));
  }, [builder]);

  return (
    <div style={style} className="flex flex-col items-center .cancelDraggEvent">
      <div className="flex flex-row justify-between items-center p-2 w-full">
        <p className="text-normal ml-4">{content.chartType}</p>
        <PanelMenu
          onOpenQueryConfig={() => setOpenQueryConfig(true)}
          onDelete={() => onDelete(id)}
        ></PanelMenu>
      </div>
      {
        <PanelConfiguration
          builder={builder}
          onConfirm={onChange}
          openFromOutside={openQueryConfig}
          onClose={() => setOpenQueryConfig(false)}
        ></PanelConfiguration>
      }
      {!loading && !resizing && (
        <VisualizationRenderer
          visualizationProps={{
            chartData: { data, userQuery: builder.build() },
          }}
          chartType={content.chartType!}
        ></VisualizationRenderer>
      )}
    </div>
  );
}
