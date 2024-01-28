import React, { useState } from "react";
import DflowGrid, { heightInPixels, widthInPixels } from "./dflow-grid";
import { DbMenu } from "./db-menu";
import { DataModel } from "../../model/data-model";
import { UserQueryBuilder } from "../../model/user-query";
import { Layout } from "react-grid-layout";
import { PanelProps } from "./panel/panel";
import { v4 } from "uuid";

export interface PanelWraper {
  layout: Layout;
  props: PanelProps;
}

function panelWrapperFactory(builder: UserQueryBuilder): PanelWraper {
  const id = v4();
  return {
    layout: {
      i: id,
      x: 0,
      y: 3,
      w: 10,
      h: 10,
      minH: 5,
      minW: 5,
      isBounded: false

    },
    props: {
      builder: builder,
      name: "PanelName: TODO",
      id: id,
      content: {
        type: "chart",
        chartType: "line",
      },
      width: widthInPixels(10),
      height: heightInPixels(10),
    },
  };
}

export default function Dashboard({ model }: { model: DataModel }) {
  const [panelWrappers, setPanelWrappers] = useState<PanelWraper[]>([]);

  const handleResize = (layout: Layout[]) => {
    setPanelWrappers((old) => {
      return old.map(item => {
        let found = layout.find(i => i.i === item.layout.i) as Layout;
        item.layout = found;
        return item
      })
    });
  };

  const addPanelWrapper = (builder: UserQueryBuilder) => {
    const newWrapper = panelWrapperFactory(builder);
    setPanelWrappers((old) => {
      return [...old, newWrapper];
    });
  };

  return (
    <div className="h-full p-1 m-2 flex justify-center">
      <div className="flex flex-col gap-2">
        <DbMenu model={model} onAddPanel={addPanelWrapper}></DbMenu>
        <DflowGrid
          wrappers={panelWrappers}
          handleResize={handleResize}
        ></DflowGrid>
      </div>
    </div>
  );
}
