'use client';
import { ImStack, ImHome } from "react-icons/im";
import { IoMdStats } from "react-icons/io";

export default function NavBar() {
  const iconClass =
    "hover:text-sky-400 text-2xl text-slate-600 duration-200 text-2xl mb-4";
  return (
    <div className="min-h-screen w-12 p-1 flex flex-col items-center bg-zinc-100">
      <IoMdStats
        className={iconClass + " mt-2 cursor-pointer"}
        onClick={() => void 0}
      />
      <ImStack
        className={iconClass + " cursor-pointer"}
        onClick={() => void 0}
      />
    </div>
  );
}
