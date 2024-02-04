"use client";
import { useEffect, useState } from "react";
import { DataModel, Table } from "../../model/data-model";
import { UserQueryBuilder } from "./model/user-query";
import { MainTableSelector } from "./selectors/tables/main-table-selector";
import { Blend, Sigma } from "lucide-react";
import { v4 } from "uuid";
import { JoinModule } from "./selectors/join/join-module";
import SummarizeModule from "./selectors/summarize/summarize-module";

import { postQuery } from "./services/query";
import { PreviewTable } from "./preview-table";
import { UserQueryState } from "./services/query-from-builder";
import { ActionButton } from "./actions/action-button";

export function UserQuery({
  model,
  queryBuilder,
  display,
}: {
  model: DataModel;
  queryBuilder: UserQueryBuilder;
  display: boolean;
}) {
  const [mainTable, setMainTable] = useState<Table>();
  const [joinModules, setJoinModules] = useState<string[]>([]);
  const [summarizeModules, setSummarizeModules] = useState<string[]>([]);
  const [showPreview, setShowPreview] = useState(false);
  const [userQueryState, setUserQueryState] = useState<
    UserQueryState | undefined
  >();

  useEffect(() => {
    const state = queryBuilder.userQueryState();
    setUserQueryState(state);
    setMainTable(state?.mainTable);
    setJoinModules(state?.joinModules.map((m) => m.id) || []);
    setSummarizeModules(state?.aggregationModules.map((m) => m.id) || []);
  }, [queryBuilder]);

  const [preview, setPreview] = useState<{
    columns: Array<string>;
    data: Array<Array<number | string>>;
  }>({ columns: [], data: [] });

  const resetQuery = () => {
    setJoinModules([]);
    setSummarizeModules([]);
    queryBuilder.reset();
    const state = queryBuilder.userQueryState();
    setUserQueryState(state);
    setShowPreview(false);
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
    queryBuilder.removeJoin(id);
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
    postQuery(user_query)
      .then((res) => {
        setPreview({ columns: res.columns, data: res.data.slice(0, 4) });
        setShowPreview(true);
      })
      .catch((e) => console.log(e));
  };
  return (
    <div
      className="flex flex-col gap-2 min-h-90"
      style={{ display: display ? "flex" : "none" }}
    >
      <MainTableSelector
        model={model}
        builder={queryBuilder}
        onTableSelect={onMainTableSelect}
        onPreview={onPreview}
        defaultValue={userQueryState}
      ></MainTableSelector>

      {mainTable &&
        joinModules.map((id) => (
          <JoinModule
            onDelete={deleteJoinModule}
            key={id}
            id={id}
            builder={queryBuilder}
            model={model}
            defaultValue={userQueryState?.joinModules.find((s) => s.id === id)}
          ></JoinModule>
        ))}
      <ActionButton
        className="h-4 w-4 text-violet-600"
        label="Join data"
        onClick={addJoinModule}
        icon={() => <Blend className="h-4 w-4 text-violet-600"/>}
      ></ActionButton>
      {mainTable &&
        summarizeModules.map((id) => (
          <SummarizeModule
            onDeleteModule={deleteSummarizeModule}
            key={id}
            id={id}
            columns={queryBuilder.getSumarizableColumns()}
            builder={queryBuilder}
            defaultValue={userQueryState?.aggregationModules.find(
              (s) => s.id === id
            )}
          ></SummarizeModule>
        ))}
      <ActionButton
        className="h-4 w-4 text-amber-600"
        label="Summarize"
        onClick={addSummarizeModule}
        icon={() => <Sigma className="h-4 w-4 text-amber-600"/>}
      ></ActionButton>
      {showPreview && (
        <PreviewTable
          columns={preview.columns}
          values={preview.data}
          onClose={() => setShowPreview(false)}
        ></PreviewTable>
      )}
    </div>
  );
}
