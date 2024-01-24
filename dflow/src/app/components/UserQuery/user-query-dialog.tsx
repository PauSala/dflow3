"use client";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Column, DataModel, Table } from "../../model/data-model";
import { TableSelector } from "./tables/table-selector";
import { ColumnSelector } from "./columns/column-selector";
import { useState } from "react";
import { Blend, Play } from "lucide-react";
import { userQuery } from "../../services/query";
import { PreviewTable } from "./preview-table";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "../../../components/ui/tooltip";
import JoinTables from "./joins/join-tables";
import Aggregation from "./aggregation/aggregation";

export function UserQueryDialog({ model }: { model: DataModel }) {
  const [selectedTable, setSelectedTable] = useState<Table>();
  const [selectedCols, setSelectedCols] = useState<
    { column: Column; table: Table }[]
  >([]);
  const [preview, setPreview] = useState<{
    columns: Array<string>;
    data: Array<Array<number | string>>;
  }>({ columns: [], data: [] });
  const [join, setJoin] = useState(false);
  const [summarize, setSummarize] = useState<{
    column: Column;
    table: Table;
    aggregation: string;
  }>();

  const [showPreview, setShowPreview] = useState(true);

  const onSelectTable = (table: Table) => {
    setSelectedTable(table);
  };

  const onSelectColumn = (columns: Column[]) => {
    setSelectedCols(
      columns.map((c) => ({ column: c, table: selectedTable as Table }))
    );
  };

  const onSelectJoinColumn = (columns: Column[], table: Table) => {
    let joinColumns = columns.map((c) => ({ column: c, table: table }));
    setSelectedCols((prev) => {
      joinColumns = joinColumns.filter(
        (c) =>
          !prev
            .map((p) => `${p.column.column_id}-${p.table.table_id}`)
            .includes(`${c.column.column_id}-${c.table.table_id}`)
      );
      let newC = [...prev, ...joinColumns];
      return newC;
    });
  };

  const onSummarize = (summarize: {
    column: Column;
    table: Table;
    aggregation: string;
  }) => {
    setSummarize(summarize);
  };

  const previewQuery = () => {
    if (selectedCols.length > 0) {
      userQuery(selectedCols, summarize!).then((res) => {
        setPreview({ columns: res.columns, data: res.data.slice(0, 4) });
        setShowPreview(true);
      });
    }
  };

  const cleanUp = () => {
    setTimeout(() => {
      setSelectedTable(undefined);
      setSelectedCols([]);
      setPreview({ columns: [], data: [] });
    });
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Query Model</Button>
      </DialogTrigger>
      <DialogContent className="min-w-[50rem]">
        <DialogHeader>
          <DialogTitle className="text-slate-700">
            {model.id} DataModel
          </DialogTitle>
          <DialogDescription>Select some table to start</DialogDescription>
        </DialogHeader>
        <div className="flex justify-start p-2 bg-emerald-100 rounded">
          <TableSelector
            tableMap={model.tables}
            onSelect={onSelectTable}
          ></TableSelector>
          {selectedTable && (
            <ColumnSelector
              columnMap={selectedTable.columns}
              onColumnSelect={onSelectColumn}
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
        {selectedTable && join && (
          <div className="flex justify-start p-2 bg-violet-100 rounded">
            <JoinTables
              prevTable={selectedTable}
              dataModel={model}
              closeJoin={() => setJoin(false)}
              onColumnSelect={onSelectJoinColumn}
            ></JoinTables>
          </div>
        )}
        <TooltipProvider delayDuration={200}>
          <Tooltip>
            {
              <TooltipTrigger asChild>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setJoin(true)}
                >
                  <Blend className="h-4 w-4 text-violet-600" />
                </Button>
              </TooltipTrigger>
            }
            {
              <TooltipContent>
                <p>Join data</p>
              </TooltipContent>
            }
          </Tooltip>
        </TooltipProvider>
        {
          <div className="flex justify-start  p-2 bg-amber-100 rounded">
            <Aggregation
              columns={selectedCols.filter(
                (c) => c.column.type_alias === "Number"
              )}
              onSummarize={onSummarize}
            ></Aggregation>
          </div>
        }
        <div className="pl-4 pr-4 max-w-[46rem]">
          {showPreview && (
            <PreviewTable
              columns={preview.columns}
              values={preview.data}
              onClose={() => setShowPreview(false)}
            ></PreviewTable>
          )}
        </div>
        <DialogFooter className="sm:justify-start">
          <DialogClose asChild>
            <Button
              type="button"
              variant="secondary"
              onClick={() => cleanUp()}
              className="w-20"
            >
              Close
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
