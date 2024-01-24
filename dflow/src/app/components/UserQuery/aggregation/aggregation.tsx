import { Sigma } from "lucide-react";
import React, { useState } from "react";
import { Column, Table } from "../../../model/data-model";
import { AggregationColumnSelector } from "./aggregation-column-selector";
import { Aggregation, AggregationSelector } from "./aggregation-selector";

export default function Aggregation({
  columns,
  onSummarize,
}: {
  columns: { column: Column; table: Table }[];
  onSummarize: (summarize: {
    column: Column;
    table: Table;
    aggregation: string;
  }) => void;
}) {
  let [selection, setSelection] = useState<{ column: Column; table: Table }>();

  return (
    <div className="flex items-center gap-2">
      <Sigma className="text-amber-600" />
      <AggregationColumnSelector
        columns={columns}
        onSelect={(col: { column: Column; table: Table }) => setSelection(col)}
      ></AggregationColumnSelector>
      {selection !== undefined && (
        <AggregationSelector
          onSelect={(agg: Aggregation) =>
            onSummarize({
              column: selection!.column,
              table: selection!.table,
              aggregation: agg.value,
            })
          }
        ></AggregationSelector>
      )}
    </div>
  );
}
