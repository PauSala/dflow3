import { useState } from "react";
import { DataModel, Table } from "../../model/data-model";
import { UserQuery, UserQueryBuilder } from "../../model/user-query";
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
import { Blend, Sigma } from "lucide-react";
import { v4 } from "uuid";
import { JoinModule } from "./selectors/join/join-module";
import SummarizeModule from "./selectors/summarize/summarize-module";

import { query } from "../../services/query";
import { PreviewTable } from "./preview-table";

export function UserQueryDialog({ model }: { model: DataModel }) {
  const [queryBuilder, setQueryBuilder] = useState<UserQueryBuilder>(
    new UserQueryBuilder(model, "test", "test")
  );

  const [mainTable, setMainTable] = useState<Table>();
  const [joinModules, setJoinModules] = useState<string[]>([]);
  const [summarizeModules, setSummarizeModules] = useState<string[]>([]);
  const [showPreview, setShowPreview] = useState(true);

  const [preview, setPreview] = useState<{
    columns: Array<string>;
    data: Array<Array<number | string>>;
  }>({ columns: [], data: [] });

  const resetQuery = () => {
    setTimeout(() => {
      setJoinModules([]);
      setSummarizeModules([]);
      setQueryBuilder(new UserQueryBuilder(model, "test", "test"));
    });
  };

  const onMainTableSelect = (table: Table) => {
    if (mainTable) {
      resetQuery();
    }
    setMainTable(table);
  };

  const addJoinModule = () => {
    let newModule = v4();
    setJoinModules((old) => {
      old.push(newModule);
      return [...old];
    });
  };

  const addSummarizeModule = () => {
    let newModule = v4();
    setSummarizeModules((old) => {
      old.push(newModule);
      return [...old];
    });
  };

  const deleteJoinModule = (id: string) => {
    setJoinModules((old) => {
      let newModules = old.filter((m) => m !== id);
      return newModules;
    });
  };

  const deleteSummarizeModule = (id: string) => {
    setSummarizeModules((old) => {
      let newModules = old.filter((m) => m !== id);
      return newModules;
    });
  };

  const onPreview = () => {
    const user_query = queryBuilder.build();
    console.log(user_query);
    query(user_query)
      .then((res) => {
        setPreview({ columns: res.columns, data: res.data.slice(0, 4) });
        setShowPreview(true);
      })
      .catch((e) => console.log(e));
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Query Model</Button>
      </DialogTrigger>
      <DialogContent className="min-w-[60rem]">
        <DialogHeader>
          <DialogTitle className="text-slate-700">
            {model.id} DataModel
          </DialogTitle>
          <DialogDescription>Select some table to start</DialogDescription>
        </DialogHeader>
        <MainTableSelector
          model={model}
          builder={queryBuilder}
          onTableSelect={onMainTableSelect}
          onPreview={onPreview}
        ></MainTableSelector>

        {mainTable &&
          joinModules.map((id) => (
            <JoinModule
              onDelete={deleteJoinModule}
              key={id}
              id={id}
              builder={queryBuilder}
              model={model}
            ></JoinModule>
          ))}

        <TooltipProvider delayDuration={100}>
          <Tooltip>
            {
              <TooltipTrigger asChild>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => addJoinModule()}
                >
                  <Blend className="h-4 w-4 text-violet-600" />
                </Button>
              </TooltipTrigger>
            }
            {
              <TooltipContent>
                <p>Join data</p>
              </TooltipContent>
            }
          </Tooltip>
        </TooltipProvider>
        {mainTable &&
          summarizeModules.map((id) => (
            <SummarizeModule
              onDeleteModule={deleteSummarizeModule}
              key={id}
              id={id}
              columns={queryBuilder.getSumarizableColumns()}
              builder={queryBuilder}
            ></SummarizeModule>
          ))}
        <TooltipProvider delayDuration={100}>
          <Tooltip>
            {
              <TooltipTrigger asChild>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => addSummarizeModule()}
                >
                  <Sigma className="h-4 w-4 text-amber-600" />
                </Button>
              </TooltipTrigger>
            }
            {
              <TooltipContent>
                <p>Summarize</p>
              </TooltipContent>
            }
          </Tooltip>
        </TooltipProvider>
        {showPreview && (
          <PreviewTable
            columns={preview.columns}
            values={preview.data}
            onClose={() => setShowPreview(false)}
          ></PreviewTable>
        )}
        <DialogFooter className="sm:justify-start">
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
