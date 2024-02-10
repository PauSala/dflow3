"use client";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { VisualizationWrapperProps } from "../chart-renderer";
import { ScrollArea } from "../../../../components/ui/scroll-area";
import { queryToLabelsValues } from "../../user-query/services/data-mapping/mappers";

export function DFlowTable({ chartData }: VisualizationWrapperProps) {
  const parsed = queryToLabelsValues(chartData.data); 
  const columns: Array<string> = parsed.labels;
  const values: Array<Array<number | string>> = parsed.values;

  return (
    <ScrollArea className="border border-slate-50 rounded-md p-4 shadow-md overflow-auto" style={{width: "90%", height: "75%"}}>
      <Table >
        <TableHeader>
          <TableRow >
            {columns.map((col, i) => (
              <TableHead key={i} className="w-[100px] font-semibold bg-zinc-100">
                {col}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {values.map((value, index) => (
            <TableRow key={index}>
              {columns.map((_, i) => (
                <TableCell
                  key={i}
                  className="text-xs whitespace-nowrap text-ellipsis overflow-hidden w-[100px]"
                >
                  {value[i]}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </ScrollArea>
  );
}
