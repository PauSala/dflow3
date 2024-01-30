'use client'
import React, { useEffect, useState } from "react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { ArrowLeftCircle, Eye, MoreVertical, Plus } from "lucide-react";
import { UserQueryBuilder } from "../../../../model/user-query";
import { ChartType } from "../../../visualizations/types";
import { Separator } from "../../../../../components/ui/separator";
import { UserQuery } from "../../../user-query/user-query";
import Visualization from "../../../user-query/visualization/visualization";

export default function PanelConfiguration({
  builder,
  onConfirm
}: {
  builder: UserQueryBuilder;
  onConfirm: (builder: UserQueryBuilder, chartType: ChartType) => void
}) {
  const [open, setOpen] = useState(false);
  const [visualize, setVisualize] = useState(false);
  const [chartType, setChartType] = useState<ChartType>("table");

  const model = builder.getModel();
  const [queryBuilder, setQueryBuilder] = useState<UserQueryBuilder>();

  //Set a new instance in order to avoid re-render and handle builderChanges
  useEffect(() => {
    setQueryBuilder(builder.newInstance());
  }, [builder]);

  const resetState = () => {
    setQueryBuilder(builder.newInstance())
    setVisualize(false);
  };

  const onDone = () => {
    setOpen(false);
    if(queryBuilder){
      onConfirm(queryBuilder, chartType);
    }
    resetState();
  };
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button
          variant="ghost"
          className="h-7 rounded cancelDraggEvent"
        >
          <MoreVertical className="w-4 h-4 text-zinc-600" />
        </Button>
      </DialogTrigger>
      <DialogContent className="min-w-[60rem] max-h-[90vh] overflow-auto cancelDraggEvent">
        <DialogHeader>
          <DialogTitle className="text-slate-700">
            {model.id} DataModel
          </DialogTitle>
        </DialogHeader>
        <Separator className="my-2" />

        {queryBuilder && (
          <UserQuery
            display={!visualize}
            model={model}
            queryBuilder={queryBuilder}
          ></UserQuery>
        )}

        {visualize && queryBuilder && (
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
