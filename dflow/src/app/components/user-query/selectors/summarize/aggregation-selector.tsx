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
import { AggregationValue } from "../../../../model/user-query";

export type AggregationT = {
  value: "Sum" | "Min" | "Max" | "Avg" | "Count" | "CountDistinct";
};

export function AggregationSelector({
  onSelect,
  defaultValue
}: {
  onSelect: (table: AggregationT) => void;
  defaultValue?: AggregationValue
}) {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");

  useEffect(()=> {
    if(defaultValue){
      setValue(defaultValue.toLowerCase());
    }
  }, [defaultValue]);

  const aggregationTypes: AggregationT[] = [
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
              ? aggregationTypes.find(
                  (table) => table.value.toLowerCase() === value
                )?.value
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
              {aggregationTypes.map((agg) => (
                <CommandItem
                  key={agg.value}
                  value={agg.value}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? "" : currentValue);
                    onSelect(
                      (aggregationTypes.find(
                        (t) =>
                          t.value.toLocaleLowerCase() === currentValue &&
                          currentValue !== value
                      ) as AggregationT) || { value: null }
                    );
                    setOpen(false);
                  }}
                >
                  <p className="w-[18rem] whitespace-nowrap text-ellipsis overflow-hidden">
                    {agg.value}
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
