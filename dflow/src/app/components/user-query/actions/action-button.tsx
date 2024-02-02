import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Button } from "@/components/ui/button";
import { Sigma } from "lucide-react";

export function ActionButton({
  className,
  label,
  onClick,
}: {
  className: string;
  label: string;
  onClick: () => void;
}) {
  return (
    <TooltipProvider delayDuration={100}>
      <Tooltip>
        {
          <TooltipTrigger asChild>
            <Button variant="outline" size="icon" onClick={() => onClick()}>
              <Sigma className={className} />
            </Button>
          </TooltipTrigger>
        }
        {
          <TooltipContent>
            <p>{label}</p>
          </TooltipContent>
        }
      </Tooltip>
    </TooltipProvider>
  );
}
