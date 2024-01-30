"use client";
import { useState } from "react";
import { DataModel } from "../../model/data-model";
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
import { UserQuery } from "./user-query";
import { ArrowLeftCircle, Eye, Plus } from "lucide-react";
import Visualization from "./visualization/visualization";
import { Separator } from "../../../components/ui/separator";
import { ChartType } from "../visualizations/types";

export function UserQueryModal({
  model,
  onConfirm,
}: {
  model: DataModel;
  onConfirm: (builder: UserQueryBuilder, chartType: ChartType) => void;
}) {
  const [open, setOpen] = useState(false);
  const [queryBuilder, setQueryBuilder] = useState<UserQueryBuilder>(
    new UserQueryBuilder(model, "test", "test")
  );
  const [visualize, setVisualize] = useState(false);
  const [chartType, setChartType] = useState<ChartType>("table");

  const resetState = () => {
    setVisualize(false);
    setQueryBuilder(new UserQueryBuilder(model, "test", "test"));
  };

  const onDone = () => {
    onConfirm(queryBuilder, chartType);
    setOpen(false);
    resetState();
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="ghost" className="h-7 rounded">
          <Plus className="mr-2 h-3 w-3" /> Add panel
        </Button>
      </DialogTrigger>
      <DialogContent className="min-w-[60rem] max-h-[90vh] overflow-auto">
        <DialogHeader>
          <DialogTitle className="text-slate-700">
            {model.id} DataModel
          </DialogTitle>
        </DialogHeader>
        <Separator className="my-2" />

        <UserQuery
          display={!visualize}
          model={model}
          queryBuilder={queryBuilder}
        ></UserQuery>

        {visualize && (
          <Visualization
            queryBuilder={queryBuilder}
            onChartType={(ct: ChartType) => {
              setChartType(ct);
            }}
          ></Visualization>
        )}
        <DialogFooter className="sm:justify-start">
          {!visualize && (
            <Button onClick={() => setVisualize((old) => !old)}>
              <Eye className="mr-2 w-4 h-4" /> Visualize
            </Button>
          )}
          {visualize && (
            <Button onClick={() => setVisualize((old) => !old)}>
              <ArrowLeftCircle className="mr-2 w-4 h-4" /> Back to query
            </Button>
          )}
          {visualize && (
            <Button type="submit" variant="secondary" onClick={() => onDone()}>
              Done
            </Button>
          )}
          <DialogClose asChild>
            <Button
              type="button"
              variant="secondary"
              className="w-20"
              onClick={() => resetState()}
            >
              Close
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
