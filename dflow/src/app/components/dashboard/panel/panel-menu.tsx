import React from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import { MoreVertical, Pencil, Trash2 } from "lucide-react";

export default function PanelMenu({
  onOpenQueryConfig,
  onDelete
}: {
  onOpenQueryConfig: () => void;
  onDelete: () => void
}) {
  return (
    <DropdownMenu >
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="h-7 rounded cancelDraggEvent">
          <MoreVertical className="w-4 h-4 text-zinc-600" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem>
          <Button
            variant="ghost"
            className="h-7 rounded"
            onClick={() => onOpenQueryConfig()}
          >
            <Pencil className="mr-2 h-4 w-4" /> Edit query
          </Button>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <Button
            variant="ghost"
            className="h-7 rounded"
            onClick={() => onDelete()}
          >
            <Trash2 className="mr-2 h-4 w-4" /> Delete
          </Button>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
