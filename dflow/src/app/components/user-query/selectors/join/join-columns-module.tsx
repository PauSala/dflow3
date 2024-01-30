"use client";
import { Equal } from "lucide-react";
import { DataModel } from "../../../../model/data-model";
import { JoinColumnsSelector } from "./join-columns";
import { useEffect, useState } from "react";
import { JoinModulesState } from "../../services/query-from-builder";

export function JoinColumnsModule({
  model,
  onFullSelection,
  filter,
  defaultValue,
}: {
  model: DataModel;
  filter: number;
  onFullSelection: (
    table1: number,
    col1: number,
    table2: number,
    col2: number
  ) => void;
  defaultValue?: JoinModulesState;
}) {
  const [selection, setSelection] = useState<{
    table1?: number;
    col1?: number;
    table2?: number;
    col2?: number;
  }>();

  useEffect(() => {
    if (defaultValue) {
      setSelection({
        table1: defaultValue.mainTable.table_id,
        col1: defaultValue.joinDefinition.mainTableColumn.column_id,
        table2: defaultValue.joinDefinition.joinTable.table_id,
        col2: defaultValue.joinDefinition.joinColumn.column_id,
      });
    }
  }, [defaultValue]);

  const set = (
    col1?: number,
    table1?: number,
    col2?: number,
    table2?: number
  ) => {
    if (col1 !== undefined && table1 !== undefined) {
      if (selection?.table2 !== undefined && selection.col2 !== undefined) {
        onFullSelection(table1, col1, selection.table2, selection.col2);
      }
      setSelection((prev) => ({
        col1,
        table1,
        col2: prev?.col2,
        table2: prev?.table2,
      }));
    }
    if (col2 !== undefined && table2 !== undefined) {
      if (selection?.table1 !== undefined && selection?.col1 !== undefined) {
        onFullSelection(selection.table1, selection.col1, table2, col2);
      }
      setSelection((prev) => ({
        col2,
        table2,
        col1: prev?.col1,
        table1: prev?.table1,
      }));
    }
  };

  return (
    <div className="flex items-center gap-2 p-1 border rounded-md bg-violet-300">
      <JoinColumnsSelector
        model={model}
        filter={filter}
        onSelectColumn={(col, table) => set(col, table, undefined, undefined)}
        defaultValue={
          defaultValue
            ? {
                table: defaultValue.mainTable,
                column: defaultValue.joinDefinition.mainTableColumn,
              }
            : void 0
        }
      ></JoinColumnsSelector>
      <Equal className="h-4 w-4 text-violet-50" />
      <JoinColumnsSelector
        model={model}
        onSelectColumn={(col, table) => set(undefined, undefined, col, table)}
        defaultValue={
          defaultValue
            ? {
                table: defaultValue.joinDefinition.joinTable,
                column: defaultValue.joinDefinition.joinColumn,
              }
            : void 0
        }
      ></JoinColumnsSelector>
    </div>
  );
}
