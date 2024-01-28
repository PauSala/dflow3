"use client";
import { ThemeProvider } from "@material-tailwind/react";
import NavBar from "./ui/nav-bar";
import { getModel } from "../services/model";
import { Button } from "../../components/ui/button";
import { useEffect, useState } from "react";
import { DataModel } from "../model/data-model";
import { UserQueryDialog } from "./user-query/user-query-dialog";
import DflowGrid from "./dashboard/dflow-grid";
import Dashboard from "./dashboard/dashboard";

export function Container() {
  let [model, setModel] = useState<DataModel>();
  const loadModel = () => {
    getModel().then((model) => setModel(model));
  };

  useEffect(() => {
    const loadModel = () => {
      getModel().then((model) => setModel(model));
    };
    loadModel();
  })

  return (
    <ThemeProvider>
      <div className="flex h-full">
        <NavBar></NavBar>
        <div className="flex flex-col w-full">
          {model && <Dashboard model={model}></Dashboard>}
        </div>
      </div>
    </ThemeProvider>
  );
}
