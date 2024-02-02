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
import { Table } from "../../../../model/data-model";

export function TableSelector({
  tableMap,
  onSelect,
  defaultValue,
  disabled
}: {
  tableMap: Record<string, Table>;
  onSelect: (table: Table) => void;
  defaultValue?: string;
  disabled: boolean
}) {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");

  const tables = Object.values(tableMap);
  useEffect(() => {
    if (defaultValue) {
      setValue(defaultValue.toLowerCase());
    }
  }, [defaultValue]);

  return (
    <Popover open={open} onOpenChange={setOpen} modal={true}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[18rem] justify-between"
          disabled={disabled}
        >
          <ChevronsUpDown className="mr-2 h-4 w-4 shrink-0 opacity-50" />
          <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
            {value
              ? tables.find((table) => table.name.toLowerCase() === value)
                  ?.display_name
              : "Select table"}
          </p>
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[18rem]">
        <Command>
          <CommandInput placeholder="Search" />
          <CommandList>
            <CommandEmpty>No table found. </CommandEmpty>
            <CommandGroup heading="Suggestions">
              {tables.map((table) => (
                <CommandItem
                  key={table.table_id}
                  value={table.name}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? "" : currentValue);
                    onSelect(
                      tables.find(
                        (t) => t.name.toLocaleLowerCase() === currentValue
                      ) as Table
                    );
                    setOpen(false);
                  }}
                >
                  <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
                    {table.display_name}
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
