import { Blend, X } from "lucide-react";
import { Button } from "../../../../../components/ui/button";
import { ColumnSelector } from "../columns/column-selector";
import { TableSelector } from "../tables/table-selector";
import { useState } from "react";
import { Column, DataModel, Table } from "../../../../model/data-model";
import { UserQueryBuilder } from "../../../../model/user-query";
import { JoinColumnsSelector } from "./join-columns";
import { JoinColumnsModule } from "./join-columns-module";

export function JoinModule({
  model,
  builder,
  id,
  onDelete,
}: {
  model: DataModel;
  builder: UserQueryBuilder;
  id: string;
  onDelete: (id: string) => void;
}) {
  const tables = model.tables;
  const [selectedTable, setSelectedTable] = useState<Table>();

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
    onDelete(id);
  };

  return (
    <div className="relative flex justify-start items-center p-2 gap-2 bg-violet-100 rounded h-[4.5em]">
      <Button variant="ghost" size="icon" onClick={() => console.log("Close!")}>
        <Blend className="h-4 w-4 text-violet-600" />
      </Button>
      <div className="flex items-center">
        <TableSelector
          tableMap={tables}
          onSelect={onSelectTable}
        ></TableSelector>
        {selectedTable && (
          <ColumnSelector
            columnMap={selectedTable.columns}
            onColumnSelect={onSelectColumn}
          ></ColumnSelector>
        )}
      </div>
      <p className="text-violet-900">On</p>
      {selectedTable && <JoinColumnsModule
        model={model}
        filter={selectedTable.table_id}
        onFullSelection={(
          table1: number,
          col1: number,
          table2: number,
          col2: number
        ) => console.log(table1, col1, table2, col2)}
      ></JoinColumnsModule>}
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
