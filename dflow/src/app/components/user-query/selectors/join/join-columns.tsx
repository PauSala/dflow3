"use client";
import { Divide, Table2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Column, DataModel, Table } from "../../../../model/data-model";
import ColumnIcon from "../columns/column-icon";
import { useEffect, useState } from "react";

function Selection({ table, column }: { table: Table; column: Column }) {
  return (
    <div className="p-4">
      <p className="text-xs font-light whitespace-nowrap text-ellipsis overflow-hidden">
        {table.display_name}
      </p>
      <p className="whitespace-nowrap text-ellipsis overflow-hidden">
        {column.display_name}
      </p>
    </div>
  );
}

export function JoinColumnsSelector({
  model,
  filter,
  onSelectColumn,
  defaultValue,
}: {
  model: DataModel;
  filter?: number;
  onSelectColumn: (column_id: number, table_id: number) => void;
  defaultValue?: {
    table: Table;
    column: Column;
  };
}) {
  const [selection, setSelection] = useState<{
    table: Table;
    column: Column;
  }>();
  useEffect(() => {
    if (defaultValue) {
      setSelection(defaultValue);
    }
  }, [defaultValue]);
  const tables = Object.values(model.tables).filter((t) =>
    filter ? t.table_id === filter : true
  );
  const onSelect = (column: Column, table: Table) => {
    setSelection({ table, column });
    onSelectColumn(column.column_id, table.table_id);
  };
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className="p-[1.8em]">
          {(selection && (
            <Selection
              column={selection.column}
              table={selection.table}
            ></Selection>
          )) ||
            "Pick a column"}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56 max-h-80 overflow-y-auto">
        <DropdownMenuLabel>Join Tables</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          {tables.map((table) => (
            <DropdownMenuSub key={table.table_id}>
              <DropdownMenuSubTrigger>
                <Table2 className="mr-2 h-4 w-4" />
                <span>{table.display_name}</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent className="w-56 max-h-80 overflow-y-auto">
                  {Object.values(table.columns).map((col) => (
                    <DropdownMenuItem
                      key={col.column_id}
                      onClick={() => onSelect(col, table)}
                    >
                      <ColumnIcon type={col.type_alias}></ColumnIcon>
                      <span>{col.display_name}</span>
                    </DropdownMenuItem>
                  ))}
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
          ))}
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
