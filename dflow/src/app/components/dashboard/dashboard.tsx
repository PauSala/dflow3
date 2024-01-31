"use client";
import React, { useEffect, useState } from "react";
import DflowGrid, { heightInPixels, widthInPixels } from "./dflow-grid";
import { DbMenu } from "./db-menu";
import { DataModel } from "../../model/data-model";
import { UserQueryBuilder } from "../user-query/model/user-query";
import { Layout } from "react-grid-layout";
import { PanelProps } from "./panel/panel";
import { v4 } from "uuid";
import { ChartType } from "../visualizations/types";
import { PanelDto, postPanel } from "./services/post-panel";
import { getPanel } from "./services/get-panels";

export interface PanelWraper {
  layout: Layout;
  props: PanelProps;
}

function panelWrapperFactory(
  builder: UserQueryBuilder,
  chartType: ChartType
): PanelWraper {
  const id = v4();
  return {
    layout: {
      i: id,
      x: 0,
      y: 0,
      w: 10,
      h: 10,
      minH: 5,
      minW: 5,
      isBounded: false,
    },
    props: {
      builder: builder,
      name: "PanelName: TODO",
      id: id,
      content: {
        type: "Chart",
        chartType: chartType,
      },
      width: widthInPixels(10),
      height: heightInPixels(10),
      resizing: false,
      onContentChange(builder, chartType) {},
    },
  };
}

export default function Dashboard({ model }: { model: DataModel }) {
  const [panelWrappers, setPanelWrappers] = useState<PanelWraper[]>([]);

  useEffect(() => {
    getPanel().then((data) => addPanelFromServer(data));
  }, []);

  const handleResize = (layout: Layout[]) => {
    setPanelWrappers((old) => {
      return old.map((item) => {
        let found = layout.find((i) => i.i === item.layout.i) as Layout;
        item.layout = found;
        return item;
      });
    });
  };

  const addPanelFromServer = (panelDto: PanelDto) => {
    const builder = new UserQueryBuilder(model);
    builder.fromQueryDto(panelDto.panel.query);
    const wrapper: PanelWraper = {
      layout: {
        ...panelDto.panel.layout,
        i: panelDto.panel_id,
        isBounded: false,
      },
      props: {
        content: {
          type: "Chart",
          chartType: panelDto.panel.props.content_type.Chart.chart_type,
        },
        width: widthInPixels(panelDto.panel.layout.w),
        height: heightInPixels(panelDto.panel.layout.h),
        id: panelDto.panel_id,
        name: panelDto.panel.props.name,
        onContentChange(builder, chartType) {},
        resizing: false,
        builder: builder,
      },
    };
    setPanelWrappers((old) => {
      return [...old.filter(e => e.layout.i !== wrapper.layout.i), wrapper];
    });
  };

  const addPanelWrapper = (builder: UserQueryBuilder, chartType: ChartType) => {
    const newWrapper = panelWrapperFactory(builder, chartType);
    setPanelWrappers((old) => {
      return [...old, newWrapper];
    });
  };

  const onPanelContentChange = (
    builder: UserQueryBuilder,
    chartType: ChartType,
    panelId: string
  ) => {
    setPanelWrappers((old) => {
      let found = old.find((o) => o.props.id === panelId);
      if (found) {
        found.props.content.chartType = chartType;
        found.props.builder = builder;
        return [...old.filter((o) => o.props.id !== panelId), found];
      }
      return [...old];
    });
  };

  const onDashboardSave = async () => {
    for (const panel of panelWrappers) {
      await postPanel(panel).catch((e) => console.log(e));
    }
  };

  return (
    <div className="h-full p-1 m-2 flex justify-center">
      <div className="flex flex-col gap-2">
        <DbMenu
          model={model}
          onAddPanel={addPanelWrapper}
          onSave={onDashboardSave}
        ></DbMenu>
        <DflowGrid
          wrappers={panelWrappers}
          handleResize={handleResize}
          onContentChange={onPanelContentChange}
        ></DflowGrid>
      </div>
    </div>
  );
}
