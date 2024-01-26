"use client";
import { ThemeProvider } from "@material-tailwind/react";
import NavBar from "./ui/nav-bar";
import { getModel } from "../services/model";
import { Button } from "../../components/ui/button";
import { useState } from "react";
import { DataModel } from "../model/data-model";
import { UserQueryDialog } from "./user-query/user-query-dialog";

export function Container() {
  let [model, setModel] = useState<DataModel>();
  const loadModel = () => {
    getModel().then((model) => setModel(model));
  };

  return (
    <ThemeProvider>
      <div className="flex h-full">
        <NavBar></NavBar>
        <div className="flex p-2 gap-px">
          <Button variant="outline" onClick={() => loadModel()}>
            Get Model
          </Button>
          {model && <UserQueryDialog model={model}></UserQueryDialog>}
        </div>
      </div>
    </ThemeProvider>
  );
}
