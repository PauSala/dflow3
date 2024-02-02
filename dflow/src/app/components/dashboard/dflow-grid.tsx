"use client";
import React, { useState } from "react";
import GridLayout, { Layout } from "react-grid-layout";
import { PanelWraper } from "./dashboard";
import Panel from "./panel/panel";
import { UserQueryBuilder } from "../user-query/model/user-query";
import { VisualizationType } from "../visualizations/types";

const panelStyle = {
  borderRadius: "6px",
  boxShadow:
    "rgba(0, 0, 0, 0.1) 0px 1px 3px 0px, rgba(0, 0, 0, 0.06) 0px 1px 2px 0px",
  backgroundColor: "white",
};

const cols = 48;
const width = 1800;
const rowHeight = 25;
const margin = 6; //[6, 6]
export const widthInPixels = (widthUnits: number) =>
  widthUnits * (width / cols) - 2 * margin;
export const heightInPixels = (heightUnits: number) =>
  heightUnits * rowHeight + margin * (heightUnits - 1);

export default function DflowGrid({
  wrappers,
  handleResize,
  onContentChange
}: {
  wrappers: PanelWraper[];
  handleResize: (layout: Layout[]) => void;
  onContentChange: (builder: UserQueryBuilder, chartType: VisualizationType, panelid: string) => void
}) {
  const [resizing, setResizing] = useState(false);

  return (
    <div className="w-[1800px] min-h-[85vh] border rounded-lg bg-slate-50 select-none">
      <GridLayout
        compactType={null}
        className="layout"
        layout={wrappers.map((w) => w.layout)}
        cols={cols}
        preventCollision={true}
        rowHeight={rowHeight}
        width={width}
        onResize={(layout) => handleResize(layout)}
        onResizeStart={() => setResizing(true)}
        onResizeStop={() => setResizing(false)}
        onDragStop={(layout) => handleResize(layout)}
        margin={[margin, margin]}
        draggableCancel=".cancelDraggEvent"
      >
        {wrappers.map((panel) => (
          <div key={panel.layout.i} style={panelStyle}>
            {
              <Panel
                builder={panel.props.builder}
                content={panel.props.content}
                id={panel.props.id}
                name={panel.props.name}
                key={panel.props.id}
                height={heightInPixels(panel.layout.h)}
                width={widthInPixels(panel.layout.w)}
                resizing={resizing}
                onContentChange={onContentChange}
              ></Panel>
            }
          </div>
        ))}
      </GridLayout>
    </div>
  );
}
