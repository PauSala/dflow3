'use-client'
import React, { useState } from "react";
import GridLayout, { Layout } from "react-grid-layout";
import { PanelWraper } from "./dashboard";
import Panel from "./panel/panel";

const panelStyle = {
  borderRadius: "6px",
  padding: "1em",
  boxShadow: "rgba(0, 0, 0, 0.1) 0px 1px 3px 0px, rgba(0, 0, 0, 0.06) 0px 1px 2px 0px",
  backgroundColor: "white",
};

const cols = 24;
const width = 1800;
const rowHeight = 25;
const margin = 6; //[4, 4]
export const widthInPixels = (widthUnits: number) =>
  widthUnits * (width / cols) + margin * (widthUnits - 1);
export const heightInPixels = (heightUnits: number) =>
  heightUnits * rowHeight + margin * (heightUnits - 1);

export default function DflowGrid({
  wrappers,
  handleResize,
}: {
  wrappers: PanelWraper[];
  handleResize: (layout: Layout[]) => void;
}) {
  return (
    <div className="w-[1800px] min-h-[85vh] border rounded-lg bg-emerald-50 select-none">
      <GridLayout
        compactType={"vertical"}
        className="layout"
        layout={wrappers.map((w) => w.layout)}
        cols={cols}
        rowHeight={rowHeight}
        width={width}
        onResize={(layout) => handleResize(layout)}
        onDragStop={(layout) => handleResize(layout)}
        margin={[margin, margin]}
      >
        {wrappers.map((panel) => (
          <div key={panel.layout.i} style={panelStyle}>
            <Panel
              builder={panel.props.builder}
              content={panel.props.content}
              id={panel.props.id}
              name={panel.props.name}
              key={panel.props.id}
              height={heightInPixels(panel.layout.h)}
              width={widthInPixels(panel.layout.w)}
            ></Panel>
          </div>
        ))}
      </GridLayout>
    </div>
  );
}
