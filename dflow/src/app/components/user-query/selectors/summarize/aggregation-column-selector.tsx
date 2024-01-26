import { Check, ChevronsUpDown } from "lucide-react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "../../../../../components/ui/popover";
import { useState } from "react";
import { Button } from "../../../../../components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "../../../../../components/ui/command";
import { Column, Table } from "../../../../model/data-model";
import { AggregationValue } from "../../../../model/user-query";

export function AggregationColumnSelector({
  columns,
  onSelect,
}: {
  columns: { column: Column & { agg: AggregationValue }; table: Table }[];
  onSelect: (table: { column: Column; table: Table }) => void;
}) {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");

  return (
    <Popover open={open} onOpenChange={setOpen} modal={true}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[18rem] justify-between"
        >
          <ChevronsUpDown className="mr-2 h-4 w-4 shrink-0 opacity-50" />
          <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
            {value
              ? columns.find((col) => col.column.name.toLowerCase() === value)
                  ?.column.display_name
              : "Select some column"}
          </p>
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[18rem]">
        <Command>
          <CommandInput placeholder="Search" />
          <CommandList>
            <CommandEmpty>No column found. </CommandEmpty>
            <CommandGroup heading="Suggestions">
              {columns.map((table) => (
                <CommandItem
                  key={table.column.column_id}
                  value={table.column.name}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? "" : currentValue);
                    onSelect(
                      columns.find(
                        (t) =>
                          t.column.name.toLocaleLowerCase() === currentValue
                      ) as { column: Column; table: Table }
                    );
                    setOpen(false);
                  }}
                >
                  <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
                    {table.column.display_name} ({table.table.display_name})
                  </p>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
