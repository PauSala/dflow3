"use client";
import { Blend, X } from "lucide-react";
import { Button } from "../../../../../components/ui/button";
import { ColumnSelector } from "../columns/column-selector";
import { TableSelector } from "../tables/table-selector";
import { useEffect, useState } from "react";
import { Column, DataModel, Table } from "../../../../model/data-model";
import { UserQueryBuilder } from "../../model/user-query";
import { JoinColumnsModule } from "./join-columns-module";
import { JoinModulesState } from "../../services/query-from-builder";

export function JoinModule({
  model,
  builder,
  id,
  onDelete,
  defaultValue,
}: {
  model: DataModel;
  builder: UserQueryBuilder;
  id: string;
  onDelete: (id: string) => void;
  defaultValue?: JoinModulesState;
}) {
  const tables = model.tables;
  const [selectedTable, setSelectedTable] = useState<Table>();
  useEffect(() => {
    if (defaultValue) {
      setSelectedTable(defaultValue.mainTable);
    }
  }, [defaultValue]);

  const onSelectTable = (table: Table) => {
    if (selectedTable) {
      builder.deleteTable(selectedTable.table_id);
    }
    setSelectedTable(table);
  };

  const onSelectColumn = (columns: Column[]) => {
    builder.addColumnsToTable(
      selectedTable!.table_id,
      columns.map((c) => c.column_id)
    );
  };

  const onDeleteModule = () => {
    if (selectedTable) {
      builder.deleteTable(selectedTable.table_id);
    }
    builder.removeJoin(id);
    onDelete(id);
  };

  const onFullSelection = (
    main_table_id: number,
    main_field_id: number,
    join_table_id: number,
    join_field_id: number
  ) => {
    builder.addJoin(
      {
        main_table_id,
        main_field_id,
        join_table_id,
        join_field_id,
      },
      id
    );
  };

  return (
    <div className="relative flex justify-start items-center p-2 gap-2 bg-violet-100 rounded h-[4.5em]">
      <Button variant="ghost" size="icon" >
        <Blend className="h-4 w-4 text-violet-600" />
      </Button>
      <div className="flex items-center">
        <TableSelector
          tableMap={tables}
          onSelect={onSelectTable}
          defaultValue={defaultValue?.mainTable.name}
        ></TableSelector>
        {selectedTable && (
          <ColumnSelector
            columnMap={selectedTable.columns}
            onColumnSelect={onSelectColumn}
            defaultValue={defaultValue?.mainTableColumns}
          ></ColumnSelector>
        )}
      </div>
      <p className="text-violet-900">on</p>
      {selectedTable && (
        <JoinColumnsModule
          model={model}
          filter={selectedTable.table_id}
          onFullSelection={onFullSelection}
          defaultValue={defaultValue}
        ></JoinColumnsModule>
      )}
      <Button
        variant="ghost"
        size="icon"
        className="absolute right-2"
        onClick={() => onDeleteModule()}
      >
        <X className="h-4 w-4 text-red-600" />
      </Button>
    </div>
  );
}
