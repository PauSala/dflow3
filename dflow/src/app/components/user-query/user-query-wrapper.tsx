import React, { useState } from "react";
import { DataModel } from "../../model/data-model";
import { UserQueryBuilder } from "../../model/user-query";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";

export default function UserQueryWrapper({
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
      <DialogContent className="min-w-[60rem] max-h-[85vh] overflow-auto"></DialogContent>
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
    </Dialog>
  );
}
