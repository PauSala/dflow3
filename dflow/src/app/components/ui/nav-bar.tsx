'use client';
import { Database, LayoutDashboard, Settings } from "lucide-react";


export default function NavBar() {
  const iconClass =
    "hover:text-emerald-600 text-2xl text-slate-600 duration-200 text-2xl mb-4";
  return (
    <div className="min-h-screen min-w-[3em] pt-3 p-1 flex flex-col items-center bg-zinc-50">
      <LayoutDashboard 
        className={iconClass + " cursor-pointer"}
      />
      <Database
        className={iconClass + " mt-2 cursor-pointer"}
      />
      <Settings  className={iconClass + " mt-2 cursor-pointer"}/>
    </div>
  );
}
