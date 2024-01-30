"use client";
import { Button } from "../../../../../components/ui/button";
import { BarChart3, LineChart, PieChart, Table } from "lucide-react";
import { DrawableChartsState } from "../../visualization/visualization";
import { ChartType } from "../../../visualizations/types";

export default function ChartSelector({
  validated,
  onChange,
}: {
  validated: DrawableChartsState;
  onChange: (chartType: ChartType) => void;
}) {
  return (
    <div className="flex items-center gap-2 p-1 border rounded-md bg-stone-50">
      <Button
        variant="ghost"
        size="icon"
        disabled={!validated.table.enabled}
        onClick={() => onChange(validated.table.name)}
      >
        <Table className="text-stone-800" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        disabled={!validated.line.enabled}
        onClick={() => onChange(validated.line.name)}
      >
        <LineChart className="text-stone-800" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        disabled={!validated.bar.enabled}
        onClick={() => (onChange(validated.bar.name))}
      >
        <BarChart3 className="text-stone-800" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        disabled={!validated.pie.enabled}
        onClick={() => onChange(validated.pie.name)}
      >
        <PieChart className="text-stone-800" />
      </Button>
      {/*       <Button variant="ghost" size="icon">
        <ScatterChart className="text-stone-800" />
      </Button>
      <Button variant="ghost" size="icon">
        <SigmaSquare className="text-stone-800" />
      </Button> */}
    </div>
  );
}
