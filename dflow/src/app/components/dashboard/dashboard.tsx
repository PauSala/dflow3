"use client";
import React, { useEffect, useState } from "react";
import DflowGrid from "./dflow-grid";
import { DbMenu } from "./db-menu";
import { DataModel } from "../../model/data-model";
import { UserQueryBuilder } from "../user-query/model/user-query";
import { Layout } from "react-grid-layout";
import { PanelProps } from "./panel/panel";
import { VisualizationType } from "../visualizations/types";
import { PanelDto, postPanel } from "./services/post-panel";
import { getPanel } from "./services/get-panels";
import {
  panelWrapperFromDto,
  panelWrapperFactory,
} from "./utils/dashboard-utils";
import { getDashboardsByUser } from "./services/get-dashboards";
import { postDashboard } from "./services/post-dashboard";

export interface PanelWraper {
  layout: Layout;
  props: PanelProps;
}

export default function Dashboard({ model }: { model: DataModel }) {
  const [panelWrappers, setPanelWrappers] = useState<PanelWraper[]>([]);

  useEffect(() => {
    getDashboardsByUser().then((data) => {
      let db = data[0];
      db.config.panels.forEach((panelId) => {
        getPanel(panelId).then((panel) => addPanelFromServer(panel));
      });
    });
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
    const wrapper: PanelWraper = panelWrapperFromDto(panelDto, model);
    setPanelWrappers((old) => {
      return [...old.filter((e) => e.layout.i !== wrapper.layout.i), wrapper];
    });
  };

  const addPanelWrapper = (builder: UserQueryBuilder, chartType: VisualizationType) => {
    const newWrapper = panelWrapperFactory(builder, chartType);
    setPanelWrappers((old) => {
      return [...old, newWrapper];
    });
  };

  const onPanelContentChange = (
    builder: UserQueryBuilder,
    chartType: VisualizationType,
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
    postDashboard({
      config: { panels: panelWrappers.map((p) => p.props.id) },
      id: "test",
      model_id: "test",
      name: "test",
      user_id: "test",
    });
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
