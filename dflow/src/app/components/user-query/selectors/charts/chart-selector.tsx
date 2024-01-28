'use-client'
import React, { useEffect, useState } from "react";
import { Button } from "../../../../../components/ui/button";
import {
  BarChart3,
  LineChart,
  PieChart,
  ScatterChart,
  SigmaSquare,
  Table,
} from "lucide-react";
import { UserQueryBuilder } from "../../../../model/user-query";
import { ChartType, chartValidatorProvider } from "../../../visualizations/types";

type DrawableChartsState = {
  [T in ChartType]: boolean;
};

const defaultDrawableChartState: DrawableChartsState = {
  line: false,
  pie: false,
  bar: false,
  table: true,
  hBar: false,
};

export default function ChartSelector({
  builder,
  update,
}: {
  builder: UserQueryBuilder;
  update: boolean;
}) {
  const [validated, setValidated] = useState<DrawableChartsState>(
    defaultDrawableChartState
  );
  useEffect(() => {
    const query = builder.build();
    setValidated(() => {
      return {
        bar: chartValidatorProvider("bar")(query),
        hBar: chartValidatorProvider("hBar")(query),
        line: chartValidatorProvider("line")(query),
        pie: chartValidatorProvider("pie")(query),
        table: chartValidatorProvider("table")(query),
      };
    });
  }, [builder, update]);
  return (
    <div className="flex items-center gap-2 p-1 border rounded-md bg-stone-50">
      <Button variant="ghost" size="icon" disabled={!validated.table}>
        <Table className="text-stone-800" />
      </Button>
      <Button variant="ghost" size="icon" disabled={!validated.line}>
        <LineChart className="text-stone-800" />
      </Button>
      <Button variant="ghost" size="icon" disabled={!validated.bar}>
        <BarChart3 className="text-stone-800" />
      </Button>
      <Button variant="ghost" size="icon" disabled={!validated.pie}>
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
