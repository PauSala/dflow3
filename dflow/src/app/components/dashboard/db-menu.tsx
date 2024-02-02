"use client";
import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "@/components/ui/menubar";
import { DataModel } from "../../model/data-model";
import { UserQueryBuilder } from "../user-query/model/user-query";
import { UserQueryModal } from "../user-query/user-query-modal";
import { VisualizationType } from "../visualizations/types";
import { Button } from "@/components/ui/button";
import { Save } from "lucide-react";

export function DbMenu({
  model,
  onAddPanel,
  onSave,
}: {
  model: DataModel;
  onAddPanel: (builder: UserQueryBuilder, chartType: VisualizationType) => void;
  onSave: () => void;
}) {
  return (
    <Menubar>
      <div>
        <p className="font-normal text-cyan-800">DASHBOARD TITLE</p>
      </div>
      <div>
        <UserQueryModal model={model} onConfirm={onAddPanel}></UserQueryModal>
      </div>
      <div>
        <Button
          variant="ghost"
          className="h-7 rounded"
          onClick={() => onSave()}
        >
          <Save className="mr-2 h-4 w-4" /> Save
        </Button>
      </div>

      <MenubarMenu>
        <MenubarTrigger>Edit</MenubarTrigger>
        <MenubarContent>
          <MenubarItem>
            Redo <MenubarShortcut>⇧⌘Z</MenubarShortcut>
          </MenubarItem>
          <MenubarSeparator />
          <MenubarSub>
            <MenubarSubTrigger>Find</MenubarSubTrigger>
            <MenubarSubContent>
              <MenubarItem>Search the web</MenubarItem>
              <MenubarSeparator />
              <MenubarItem>Find...</MenubarItem>
              <MenubarItem>Find Next</MenubarItem>
              <MenubarItem>Find Previous</MenubarItem>
            </MenubarSubContent>
          </MenubarSub>
          <MenubarSeparator />
          <MenubarItem>Cut</MenubarItem>
          <MenubarItem>Copy</MenubarItem>
          <MenubarItem>Paste</MenubarItem>
        </MenubarContent>
      </MenubarMenu>
      <MenubarMenu>
        <MenubarTrigger>View</MenubarTrigger>
        <MenubarContent>
          <MenubarCheckboxItem>Always Show Bookmarks Bar</MenubarCheckboxItem>
          <MenubarCheckboxItem checked>
            Always Show Full URLs
          </MenubarCheckboxItem>
          <MenubarSeparator />
          <MenubarItem inset>
            Reload <MenubarShortcut>⌘R</MenubarShortcut>
          </MenubarItem>
          <MenubarItem disabled inset>
            Force Reload <MenubarShortcut>⇧⌘R</MenubarShortcut>
          </MenubarItem>
          <MenubarSeparator />
          <MenubarItem inset>Toggle Fullscreen</MenubarItem>
          <MenubarSeparator />
          <MenubarItem inset>Hide Sidebar</MenubarItem>
        </MenubarContent>
      </MenubarMenu>
    </Menubar>
  );
}
