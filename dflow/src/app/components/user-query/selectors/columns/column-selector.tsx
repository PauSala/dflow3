'use-client'
import { Check, ChevronsUpDown } from "lucide-react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "../../../../../components/ui/popover";
import { useEffect, useState } from "react";
import { Button } from "../../../../../components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "../../../../../components/ui/command";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "../../../../../components/ui/tooltip";
import { Column } from "../../../../model/data-model";
import { cn } from "../../../../../lib/utils";
import ColumnIcon from "./column-icon";

export function ColumnSelector({
  columnMap,
  onColumnSelect,
}: {
  columnMap: Record<string, Column>;
  onColumnSelect: (cols: Array<Column>) => void;
}) {
  const [open, setOpen] = useState(false);
  const [selection, setSelection] = useState<Column[]>([]);
  const columns = Object.values(columnMap);

  useEffect(() => {
    setSelection([]);
  }, [columnMap]);

  const onSelect = (currentValue: string) => {
    let found = columns.find(
      (c) => c.name.toLowerCase() === currentValue
    ) as Column;
    let newS = [...selection];
    if (newS.includes(found)) {
      let n = newS.filter((e) => e !== found);
      setSelection(n);
      onColumnSelect(n);
    } else {
      newS.push(found);
      setSelection(newS);
      onColumnSelect(newS);
    }
  };

  return (
    <Popover open={open} onOpenChange={setOpen} modal={true}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[2rem]"
        >
          <TooltipProvider delayDuration={200}>
            <Tooltip>
              <TooltipTrigger asChild>
                <ChevronsUpDown className="h-4 w-4 shrink-0 opacity-50" />
              </TooltipTrigger>
              <TooltipContent>
                <p>Select columns</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[15rem]" side="right" align="start">
        <Command>
          <CommandInput placeholder="Search" />
          <CommandList>
            <CommandEmpty>No column found. </CommandEmpty>
            <CommandGroup>
              {columns.map((col) => (
                <CommandItem
                  key={col.column_id}
                  value={col.name}
                  onSelect={(currentValue) => onSelect(currentValue)}
                >
                  <div className="flex">
                    <Check
                      className={cn(
                        "mr-2 h-4 w-4",
                        selection
                          .map((s) => s.column_id)
                          .includes(col.column_id)
                          ? "opacity-100"
                          : "opacity-0"
                      )}
                    />
                    <ColumnIcon type={col.type_alias}></ColumnIcon>
                    <p className="w-[8rem] whitespace-nowrap text-ellipsis overflow-hidden">
                      {col.display_name}
                    </p>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
