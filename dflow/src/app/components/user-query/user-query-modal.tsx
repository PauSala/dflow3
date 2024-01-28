"use-client";
import { useState } from "react";
import { DataModel, Table } from "../../model/data-model";
import { UserQueryBuilder } from "../../model/user-query";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../../../components/ui/dialog";
import { Button } from "../../../components/ui/button";
import { MainTableSelector } from "./selectors/tables/main-table-selector";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "../../../components/ui/tooltip";
import { Blend, Plus, Sigma } from "lucide-react";
import { v4 } from "uuid";
import { JoinModule } from "./selectors/join/join-module";
import SummarizeModule from "./selectors/summarize/summarize-module";

import { query } from "./services/query";
import { PreviewTable } from "./preview-table";
import ChartSelector from "./selectors/charts/chart-selector";

export function UserQueryModal({
  model,
  onConfirm,
}: {
  model: DataModel;
  onConfirm: (builder: UserQueryBuilder) => void;
}) {
  const [open, setOpen] = useState(false);
  const [queryBuilder, setQueryBuilder] = useState<UserQueryBuilder>(
    new UserQueryBuilder(model, "test", "test")
  );

  const resetQuery = () => {};

  const onDone = () => {
    onConfirm(queryBuilder);
    setOpen(false);
    resetQuery();
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="ghost" className="h-7 rounded">
          <Plus className="mr-2 h-3 w-3" /> Add panel
        </Button>
      </DialogTrigger>
      <DialogContent className="min-w-[60rem] max-h-[85vh] overflow-auto">
        <DialogHeader>
          <DialogTitle className="text-slate-700">
            {model.id} DataModel
          </DialogTitle>
        </DialogHeader>

        <DialogFooter className="sm:justify-start">
          <Button type="submit" onClick={() => onDone()}>
            Done
          </Button>
          <DialogClose asChild>
            <Button
              type="button"
              variant="secondary"
              onClick={() => resetQuery()}
              className="w-20"
            >
              Close
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
