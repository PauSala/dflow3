import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "../../../components/ui/button";
import { X } from "lucide-react";

export function PreviewTable(data: {
  columns: Array<string>;
  values: Array<Array<number | string>>;
  onClose: () => void
}) {
  return (
    <div className="relative border border-slate-50 rounded-md p-4 shadow-sm">
      <Button variant="ghost" className="absolute top-0 right-0" size="icon" onClick={() => data.onClose()}>
        <X className="h-4 w-4" />
      </Button>
      <p className="text-slate-600 font-semibold">Preview</p>
      <Table className="min-h-full ">
        <TableHeader>
          <TableRow>
            {data.columns.map((col, i) => (
              <TableHead key={i} className="w-[100px] font-semibold">
                {col}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.values.map((value, index) => (
            <TableRow key={index}>
              {data.columns.map((_, i) => (
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
