import { Check, ChevronsUpDown } from "lucide-react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "../../../../components/ui/popover";
import { useState } from "react";
import { Button } from "../../../../components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "../../../../components/ui/command";

export type Aggregation = {
    value: "Sum" | "Min" | "Max" | "Avg" | "Count" | "CountDistinct";
}

export function AggregationSelector({
  onSelect,
}: {
  onSelect: (table: Aggregation) => void;
}) {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");

  const aggregationTypes: Aggregation[] = [
    { value: "Sum" },
    { value: "Avg" },
    { value: "Count" },
    { value: "CountDistinct" },
    { value: "Max" },
    { value: "Min" },
  ];

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
              ? aggregationTypes.find((table) => table.value.toLowerCase() === value)
                  ?.value
              : "Summarize by"}
          </p>
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[18rem]">
        <Command>
          <CommandInput placeholder="Search" />
          <CommandList>
            <CommandEmpty>No aggregation found. </CommandEmpty>
            <CommandGroup heading="Suggestions">
              {aggregationTypes.map((table) => (
                <CommandItem
                  key={table.value}
                  value={table.value}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? "" : currentValue);
                    onSelect(
                      aggregationTypes.find(
                        (t) => t.value.toLocaleLowerCase() === currentValue
                      ) as Aggregation
                    );
                    setOpen(false);
                  }}
                >
                  <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
                    {table.value}
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
