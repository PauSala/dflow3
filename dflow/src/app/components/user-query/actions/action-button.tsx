import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Button } from "@/components/ui/button";
import { Sigma } from "lucide-react";
import { ReactNode } from "react";

export function ActionButton({
  className,
  label,
  onClick,
  icon
}: {
  className: string;
  label: string;
  onClick: () => void;
  icon: () => ReactNode
}) {
  return (
    <TooltipProvider delayDuration={100}>
      <Tooltip>
        {
          <TooltipTrigger asChild>
            <Button variant="outline" size="icon" onClick={() => onClick()}>
              {icon()}
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
