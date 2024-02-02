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

export function DFlowTable({ chartData }: VisualizationWrapperProps) {
  const columns: Array<string> = chartData.data.columns;
  const values: Array<Array<number | string>> = chartData.data.data;

  return (
    <div className="relative border border-slate-50 rounded-md p-4 shadow-md" style={{width: "90%", height: "75%"}}>
      <Table className="min-h-full ">
        <TableHeader>
          <TableRow>
            {columns.map((col, i) => (
              <TableHead key={i} className="w-[100px] font-semibold">
                {col}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {values.map((value, index) => (
            <TableRow key={index}>
              {chartData.data.columns.map((_, i) => (
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
    </div>
  );
}
