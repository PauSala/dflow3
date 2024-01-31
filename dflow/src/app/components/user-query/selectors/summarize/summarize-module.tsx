import { Sigma, X } from "lucide-react";
import React, { useEffect, useState } from "react";
import { Column, Table } from "../../../../model/data-model";
import { AggregationColumnSelector } from "./aggregation-column-selector";
import { AggregationSelector, AggregationT } from "./aggregation-selector";
import { Button } from "../../../../../components/ui/button";
import {
  AggregationValue,
  UserQueryBuilder,
} from "../../model/user-query";
import { AggregationModulesState } from "../../services/query-from-builder";

export default function SummarizeModule({
  columns,
  onDeleteModule,
  id,
  builder,
  defaultValue
}: {
  columns: { column: Column & { agg: AggregationValue }; table: Table }[];
  onDeleteModule: (id: string) => void;
  id: string;
  builder: UserQueryBuilder;
  defaultValue?: AggregationModulesState;
}) {
  const [selection, setSelection] = useState<{ column: Column; table: Table }>();

  useEffect(()=> {  
    if(defaultValue){
      setSelection(defaultValue);
    }
  }, [defaultValue])
  return (
    <div className="flex justify-start  p-2 bg-amber-100 rounded">
      <div className="flex items-center gap-2">
        <Sigma className="text-amber-600" />
        <AggregationColumnSelector
          columns={columns}
          onSelect={(col: { column: Column; table: Table }) =>
            setSelection(col)
          }
          defaultValue={defaultValue?.column.name}
        ></AggregationColumnSelector>
        {selection !== undefined && (
          <AggregationSelector
            onSelect={(agg: AggregationT) => {
              builder.addAggregation(
                selection!.table.table_id,
                selection!.column.column_id,
                agg.value
              );
            }}
            defaultValue={defaultValue?.aggregation}
          ></AggregationSelector>
        )}
        <Button
          variant="ghost"
          size="icon"
          className="absolute right-8"
          onClick={() => {
            if (selection) {
              builder.removeAggregation(
                selection!.table.table_id,
                selection!.column.column_id
              );
            }
            onDeleteModule(id);
          }}
        >
          <X className="h-4 w-4 text-red-600" />
        </Button>
      </div>
    </div>
  );
}
