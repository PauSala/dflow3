import React, { useEffect, useState } from "react";
import { Column, DataModel, Table } from "../../../model/data-model";
import { Blend } from "lucide-react";
import { Button } from "../../../../components/ui/button";
import { TableSelector } from "../tables/table-selector";
import { findRelations } from "../../../services/find-relations";
import { ColumnSelector } from "../columns/column-selector";

function CardTable({ value }: { value: string }) {
  return (
    <div className="bg-white p-[0.5em] rounded text-sm font-medium">
      <p>{value}</p>
    </div>
  );
}

export default function JoinTables({
  prevTable,
  dataModel,
  closeJoin,
  onColumnSelect,
}: {
  prevTable: Table;
  dataModel: DataModel;
  closeJoin: () => void;
  onColumnSelect: (columns: Column[], table: Table) => void;
}) {
  const [joinTables, setJoinTables] = useState<Record<string, Table>>();
  const [selectedTable, setSelectedTable] = useState<Table>();

  useEffect(() => {
    let tables = findRelations(prevTable, dataModel);
    setJoinTables(tables);
  }, [prevTable]);

  const selectColumnsHandler = (columns: Column[]) => {
    onColumnSelect(columns, selectedTable as Table);
  };

  return (
    <div className="flex flex-column items-center gap-1">
      <CardTable value={prevTable.display_name}></CardTable>
      <Button variant="ghost" size="icon" onClick={() => closeJoin()}>
        <Blend className="h-4 w-4 text-violet-600" />
      </Button>
      <div>
        {joinTables && (
          <TableSelector
            onSelect={(t: Table) => setSelectedTable(t)}
            tableMap={joinTables}
          ></TableSelector>
        )}
        {selectedTable && (
          <ColumnSelector
            columnMap={selectedTable.columns}
            onColumnSelect={selectColumnsHandler}
          ></ColumnSelector>
        )}
      </div>
    </div>
  );
}
