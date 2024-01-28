"use client";
import { ThemeProvider } from "@material-tailwind/react";
import NavBar from "./ui/nav-bar";
import { getModel } from "../services/model";
import { useEffect, useState } from "react";
import { DataModel } from "../model/data-model";
import Dashboard from "./dashboard/dashboard";

export function Container() {
  let [model, setModel] = useState<DataModel>();
  useEffect(() => {
    const loadModel = () => {
      getModel().then((model) => setModel(model));
    };
    loadModel();
  }, []);

  return (
    <ThemeProvider>
      <div className="flex h-full">
        <NavBar></NavBar>
        <div className="flex flex-grow flex-col">
          {model && <Dashboard model={model}></Dashboard>}
        </div>
      </div>
    </ThemeProvider>
  );
}
