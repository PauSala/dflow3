import { Play } from "lucide-react";
import { Button } from "../../../../../components/ui/button";
import { ColumnSelector } from "../columns/column-selector";
import { TableSelector } from "./table-selector";
import { useEffect, useState } from "react";
import { Column, DataModel, Table } from "../../../../model/data-model";
import { UserQueryBuilder } from "../../model/user-query";
import { UserQueryState } from "../../services/query-from-builder";

export function MainTableSelector({
  model,
  builder,
  onTableSelect,
  onPreview,
  defaultValue,
}: {
  model: DataModel;
  builder: UserQueryBuilder;
  onTableSelect: (table: Table) => void;
  onPreview: () => void;
  defaultValue?: UserQueryState;
}) {
  const [selectedTable, setSelectedTable] = useState<Table>();
  useEffect(() => {
    if(defaultValue){
      setSelectedTable(defaultValue.mainTable)
    }
  }, [defaultValue]);

  const onSelectTable = (table: Table) => {
    setSelectedTable(table);
    onTableSelect(table);
  };

  const onSelectColumn = (columns: Column[]) => {
    builder.addColumnsToTable(
      selectedTable!.table_id,
      columns.map((c) => c.column_id)
    );
  };
  const previewQuery = () => {
    onPreview();
  };
  return (
    <div className="flex justify-start p-2 bg-emerald-100 rounded">
      <TableSelector
        tableMap={model.tables}
        onSelect={onSelectTable}
        defaultValue={defaultValue?.mainTable?.name}
      ></TableSelector>
      {selectedTable && (
        <ColumnSelector
          columnMap={selectedTable.columns}
          onColumnSelect={onSelectColumn}
          defaultValue={defaultValue?.mainTableColumns}
        ></ColumnSelector>
      )}
      <Button
        variant="outline"
        size="icon"
        className="ml-auto"
        onClick={() => previewQuery()}
      >
        <Play className="h-4 w-4" />
      </Button>
    </div>
  );
}
