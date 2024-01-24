import { Blend, Play, X } from "lucide-react";
import { Button } from "../../../../../components/ui/button";
import { ColumnSelector } from "../columns/column-selector";
import { TableSelector } from "../tables/table-selector";
import { useState } from "react";
import { Column, DataModel, Table } from "../../../../model/data-model";
import { UserQueryBuilder } from "../../../../model/user-query";
import { findRelations } from "../../../../services/find-relations";

export function JoinModule({
  model,
  builder,
  id,
  mainTable,
  onDelete,
}: {
  model: DataModel;
  builder: UserQueryBuilder;
  id: string;
  mainTable: Table;
  onDelete: (id: string) => void;
}) {
  const tables = findRelations(mainTable, model);
  const [selectedTable, setSelectedTable] = useState<Table>();

  const onSelectTable = (table: Table) => {
    if(selectedTable){
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
    <div className="relative flex justify-start p-2 bg-violet-100 rounded">
      <Button variant="ghost" size="icon" onClick={() => console.log("Close!")}>
        <Blend className="h-4 w-4 text-violet-600" />
      </Button>
      <TableSelector tableMap={tables} onSelect={onSelectTable}></TableSelector>
      {selectedTable && (
        <ColumnSelector
          columnMap={selectedTable.columns}
          onColumnSelect={onSelectColumn}
        ></ColumnSelector>
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
